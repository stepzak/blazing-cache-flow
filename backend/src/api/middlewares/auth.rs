use std::sync::Arc;

use axum::{extract::FromRequestParts, http::{self, StatusCode, request::Parts}};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{api::state::AppState, services::auth::Claims};

pub struct UserContext {
    pub user_id: Uuid
}

impl FromRequestParts<Arc<AppState>> for UserContext
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {

        let auth_header = parts
        .headers
        .get(http::header::AUTHORIZATION)
        .and_then(|x| x.to_str().ok())
        .filter(|x| x.starts_with("Bearer "))
        .map(|x| &x[7..])
        .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid token"))?;

        let token_data = decode::<Claims>(
            auth_header,
            &DecodingKey::from_secret(state.settings.auth.secret_key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(UserContext {
            user_id: token_data.claims.user_id,
        })
    }
}