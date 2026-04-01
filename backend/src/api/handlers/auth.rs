use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_valid::Valid;

use crate::{
    api::{
        dto::auth::{LoginDTO, RegisterDTO, TokenResponse, UserResponse, ValidateEmailDTO}, middlewares::auth::UserContext, state::AppState
    },
    services::auth::AuthError,
};

/// Register user
///
/// Sends email verification
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterDTO,
    tag = "Auth",
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 409, description = "User already exists"),
        (status = 202, description = "User already exists, but not verified", body = UserResponse),
        (status = 400, description = "Invalid email")
    )
)]
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Valid(Json(request)): Valid<Json<RegisterDTO>>,
) -> Result<impl IntoResponse, AuthError> {
    let res = state.auth_service.register(request).await?;
    let user = res.user;
    let code = res.code;
    let email = user.email.clone();
    let name = user.name.clone();
    tokio::spawn(async move {
        if let Err(e) = state
            .email_sender
            .send_verification_code(&email, &name, &code)
            .await
        {
            tracing::error!("Failed to send email: {:?}", e);
        }
    });
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

#[utoipa::path(
    post,
    path = "/api/auth/verify_email",
    tag = "Auth",
    request_body = ValidateEmailDTO,
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 409, description = "Validation error"),
        (status = 400, description = "Invalid code"),
        (status = 404, description = "User not found(or has no codes)")
    )
)]
pub async fn verify_email_handler(
    State(state): State<Arc<AppState>>,
    Valid(Json(request)): Valid<Json<ValidateEmailDTO>>,
) -> Result<impl IntoResponse, AuthError> {
    let res = state
        .auth_service
        .verify_email(request.user_id, &request.code)
        .await?;
    Ok((StatusCode::OK, Json(UserResponse::from(res))))
}


#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginDTO,
    responses(
        (status = 200, description = "OK", body = TokenResponse),
        (status = 401, description = "Invalid email or password"),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Not verified")
    )
)]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Valid(Json(request)): Valid<Json<LoginDTO>>
) -> Result<impl IntoResponse, AuthError> {
    let res = state.auth_service.login(&request.username, &request.password).await?;
    Ok((StatusCode::OK, Json(res)))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "Auth",
    security(
        ("api_jwt" = [])
    ),
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    )
)]
pub async fn me_handler(
    State(state): State<Arc<AppState>>,
    user: UserContext
) -> Result<impl IntoResponse, AuthError> {
    let user = state.auth_service.get_by_id(user.user_id).await?;
    match user {
        Some(u) => Ok((StatusCode::OK, Json(UserResponse::from(u)))),
        None => Err(AuthError::NotFound)
    }
}
