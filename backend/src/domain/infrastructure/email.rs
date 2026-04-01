use async_trait::async_trait;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_verification_code(
        &self,
        to_email: &str,
        name: &str,
        code: &str,
    ) -> anyhow::Result<()>;
}
