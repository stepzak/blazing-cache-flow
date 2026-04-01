use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{api::dto::auth::UserResponse, services::auth::AuthError};

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AuthError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
            AuthError::BcryptError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Encryption failed".into(),
            ),
            AuthError::AlreadySent(user) => {
                return (StatusCode::ACCEPTED, Json(UserResponse::from(user))).into_response();
            }
            AuthError::InvalidCode => (StatusCode::BAD_REQUEST, "Invalid code".into()),
            AuthError::NotFound => (StatusCode::NOT_FOUND, "No codes or invalid user id".into()),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid creds".into()),
            AuthError::JWTError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT fault".into()),
            AuthError::NotVerified => (StatusCode::FORBIDDEN, "Not verified".into())
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
