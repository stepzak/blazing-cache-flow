use super::base::BaseFields;
use bcrypt::{BcryptError, DEFAULT_COST, hash};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub verified: bool,
}

impl User {
    pub fn new(
        email: String,
        password: &str,
        name: String,
        verified: bool,
    ) -> Result<Self, BcryptError> {
        let hashed = User::hash_password(password)?;
        Ok(User {
            base: BaseFields::new(),
            email: email,
            password_hash: hashed,
            name: name,
            verified: verified,
        })
    }

    fn hash_password(password: &str) -> Result<String, BcryptError> {
        hash(password, DEFAULT_COST)
    }
}
