use chrono::{DateTime, TimeDelta, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::base::BaseFields;

#[derive(Debug, Clone, Copy, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "emailcodeaction", rename_all = "snake_case")]
pub enum EmailCodeAction {
    ChangePassword,
    ChangeEmail,
    Register,
}

#[derive(sqlx::FromRow)]
pub struct EmailCode {
    #[sqlx(flatten)]
    pub base: BaseFields,
    code_hash: String,
    pub action: EmailCodeAction,
    pub expires_at: DateTime<Utc>,
    pub user_id: Uuid,
}

impl EmailCode {
    pub fn new(code_plain: &str, action: EmailCodeAction, user_id: Uuid, expire_min: u16) -> Self {
        let base = BaseFields::new();
        let delta = TimeDelta::minutes(expire_min.into());
        let expires_at = base.created_at.checked_add_signed(delta).unwrap();
        EmailCode {
            base,
            code_hash: EmailCode::hash_code(code_plain),
            action,
            expires_at,
            user_id,
        }
    }

    fn hash_code(code_plain: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(code_plain);
        let res = hasher.finalize();
        hex::encode(res)
    }

    pub fn verify(&self, code_plain: &str) -> bool {
        let hashed = Self::hash_code(code_plain);
        self.code_hash == hashed && self.expires_at > Utc::now()
    }

    pub fn code_hash(&self) -> &str {
        &self.code_hash
    }

    pub fn expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}
