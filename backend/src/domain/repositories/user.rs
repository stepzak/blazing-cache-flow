use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{entities::user::User, repositories::base::BaseRepository};

#[async_trait]
pub trait UserRepository: BaseRepository<User> {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
}
