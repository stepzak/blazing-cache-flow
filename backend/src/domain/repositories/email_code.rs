use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::email_code::{EmailCode, EmailCodeAction},
    repositories::base::BaseRepository,
};

#[async_trait]
pub trait EmailCodeRepository: BaseRepository<EmailCode> {
    async fn get_by_user_action(
        &self,
        user_id: Uuid,
        action: EmailCodeAction,
    ) -> anyhow::Result<Option<EmailCode>>;
    async fn delete_by_user_action(
        &self,
        user_id: Uuid,
        action: EmailCodeAction,
    ) -> anyhow::Result<()>;
}
