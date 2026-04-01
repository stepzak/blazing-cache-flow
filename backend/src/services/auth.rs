use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use thiserror::Error;

use rand::prelude::*;
use uuid::Uuid;

use crate::{
    api::dto::auth::{RegisterDTO, TokenResponse},
    config::Settings,
    domain::{
        entities::{
            email_code::{EmailCode, EmailCodeAction},
            user::User,
        },
        repositories::{email_code::EmailCodeRepository, user::UserRepository},
    },
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    email_repo: Arc<dyn EmailCodeRepository>,
    settings: Arc<Settings>,
}

pub struct RegisterResult {
    pub user: User,
    pub code: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Database error: {0}")]
    Database(#[from] anyhow::Error),

    #[error("Encryption error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Already sent email")]
    AlreadySent(User),

    #[error("No user found")]
    NotFound,

    #[error("Invalid code")]
    InvalidCode,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Not verified")]
    NotVerified,

    #[error("JWT Error")]
    JWTError(#[from] jsonwebtoken::errors::Error)
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        email_repo: Arc<dyn EmailCodeRepository>,
        settings: &Arc<Settings>,
    ) -> Self {
        AuthService {
            user_repo,
            email_repo,
            settings: Arc::clone(settings),
        }
    }

    fn generate_otp(length: usize) -> String {
        let mut rng = rand::rng();
        let max = 10u32.pow(length as u32);
        let code: u32 = rng.random_range(0..max);

        format!("{:0width$}", code, width = length as usize)
    }

    pub async fn register(&self, request: RegisterDTO) -> Result<RegisterResult, AuthError> {
        let user: User = if let Some(user_existing) =
            self.user_repo.get_by_email(&request.email).await?
        {
            match user_existing.verified {
                true => return Err(AuthError::UserAlreadyExists),
                false => {
                    let code = self
                        .email_repo
                        .get_by_user_action(user_existing.base.id, EmailCodeAction::Register)
                        .await?;
                    if let Some(c) = code {
                        if !c.expired() {
                            return Err(AuthError::AlreadySent(user_existing));
                        }

                        self.email_repo
                            .delete_by_user_action(user_existing.base.id, EmailCodeAction::Register)
                            .await?;
                    }
                }
            }

            user_existing
        } else {
            let user_create = User::new(request.email, &request.password, request.name, false)?;

            let user_new = self.user_repo.create(&user_create).await?;

            user_new
        };

        let code_plain = AuthService::generate_otp(6);

        let email_code = EmailCode::new(
            &code_plain,
            EmailCodeAction::Register,
            user.base.id,
            self.settings.email.code_expire_min,
        );

        self.email_repo.create(&email_code).await?;

        Ok(RegisterResult {
            user,
            code: code_plain,
        })
    }

    pub async fn verify_email(&self, user_id: Uuid, code: &str) -> Result<User, AuthError> {
        let user = match self.user_repo.get_by_id(user_id).await? {
            Some(us) => us,
            None => return Err(AuthError::NotFound),
        };

        if user.verified {
            return Ok(user);
        }

        let code_record = match self
            .email_repo
            .get_by_user_action(user_id, EmailCodeAction::Register)
            .await?
        {
            Some(c) => c,
            None => return Err(AuthError::NotFound),
        };

        if !code_record.verify(code) {
            return Err(AuthError::InvalidCode);
        }

        let mut verified_user = user;
        verified_user.verified = true;

        let updated = self.user_repo.update(&verified_user).await?;
        self.email_repo.delete_by_id(code_record.base.id).await?;

        Ok(updated)
    }

    fn create_token(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), claims, &EncodingKey::from_secret(self.settings.auth.secret_key.as_bytes()))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<TokenResponse, AuthError> {
        let user = self
            .user_repo
            .get_by_email(email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;
        if !user.verified {
            return Err(AuthError::NotVerified);
        }

        let pwd = user.password_hash;
        match bcrypt::verify(password, &pwd) {
            Ok(r) => {
                if !r {
                    return Err(AuthError::InvalidCredentials);
                }
            }
            Err(e) => return Err(AuthError::BcryptError(e))
        };
        let access_exp_min = self.settings.auth.access_expire_min;
        let access_exp = Utc::now() + Duration::minutes(access_exp_min);
        let access_claims = Claims {user_id: user.base.id, exp: access_exp.timestamp() as usize };
        let access = self.create_token(&access_claims)?;
        
        let refresh_exp = Utc::now() + Duration::minutes(self.settings.auth.refresh_expire_min);
        let refresh_claims = Claims {user_id: user.base.id, exp: refresh_exp.timestamp() as usize};

        let refresh = self.create_token(&refresh_claims)?;

        Ok(
            TokenResponse { access_token: access, refresh_token: refresh, token_type: "Bearer".into(), expires_in: access_exp_min * 60 }
        )
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let us = self.user_repo.get_by_id(id).await?;
        Ok(us)
    }
}
