use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::domain::{
    entities::email_code::{EmailCode, EmailCodeAction},
    repositories::{base::BaseRepository, email_code::EmailCodeRepository},
};

pub struct PostgresEmailCodeRepository {
    pool: PgPool,
}

impl PostgresEmailCodeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository<EmailCode> for PostgresEmailCodeRepository {
    async fn create(&self, data: &EmailCode) -> anyhow::Result<EmailCode> {
        let code = query_as::<_, EmailCode>(
            r#"
                INSERT INTO email_codes (id, created_at, updated_at, user_id, code_hash, action, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#
        )
        .bind(data.base.id)
        .bind(data.base.created_at)
        .bind(data.base.updated_at)
        .bind(data.user_id)
        .bind(data.code_hash())
        .bind(data.action)
        .bind(data.expires_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(code)
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<Option<EmailCode>> {
        let code = query_as::<_, EmailCode>(
            r#"
            SELECT *
            FROM email_codes
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(code)
    }

    async fn get_many(
        &self,
        skip: Option<i64>,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<EmailCode>> {
        let codes = query_as::<_, EmailCode>(
            r#"
            SELECT *
            FROM email_codes
            OFFSET $1
            LIMIT $2
            "#,
        )
        .bind(skip.unwrap_or(0))
        .bind(limit.unwrap_or(100))
        .fetch_all(&self.pool)
        .await?;

        Ok(codes)
    }

    async fn delete_by_id(&self, id: Uuid) -> anyhow::Result<()> {
        query(
            r#"
            DELETE
            FROM email_codes
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl EmailCodeRepository for PostgresEmailCodeRepository {
    async fn get_by_user_action(
        &self,
        user_id: Uuid,
        action: EmailCodeAction,
    ) -> anyhow::Result<Option<EmailCode>> {
        let code = query_as::<_, EmailCode>(
            r#"
            SELECT *
            FROM email_codes
            WHERE user_id = $1
            AND action = $2
            "#,
        )
        .bind(user_id)
        .bind(action)
        .fetch_optional(&self.pool)
        .await?;
        Ok(code)
    }

    async fn delete_by_user_action(
        &self,
        user_id: Uuid,
        action: EmailCodeAction,
    ) -> anyhow::Result<()> {
        query(
            r#"
            DELETE
            FROM email_codes
            WHERE user_id = $1
            AND action = $2
            "#,
        )
        .bind(user_id)
        .bind(action)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
