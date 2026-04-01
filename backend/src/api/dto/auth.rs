use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::user::User;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterDTO {
    #[validate(email(message = "Invalid email"))]
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "S Z")]
    pub name: String,
    #[schema(example = "qwerty")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub verified: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.base.id,
            email: user.email,
            name: user.name,
            created_at: user.base.created_at,
            verified: user.verified,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ValidateEmailDTO {
    pub user_id: Uuid,
    #[validate(length(equal = 6))]
    #[schema(example = "123456")]
    pub code: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginDTO {
    #[validate(email(message = "Invalid email"))]
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
