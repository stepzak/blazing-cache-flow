use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BaseRepository<T>: Send + Sync
where
    T: Send + Sync,
{
    async fn create(&self, data: &T) -> Result<T>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<T>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn get_many(&self, skip: Option<i64>, limit: Option<i64>) -> Result<Vec<T>>;
}
