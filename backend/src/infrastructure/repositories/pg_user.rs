use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repositories::{base::BaseRepository, user::UserRepository},
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseRepository<User> for PostgresUserRepository {
    async fn create(&self, data: &User) -> Result<User> {
        let user = query_as::<_, User>(
            r#"
            INSERT INTO users (id, created_at, updated_at, email, password_hash, name, verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(data.base.id)
        .bind(data.base.created_at)
        .bind(data.base.updated_at)
        .bind(&data.email)
        .bind(&data.password_hash)
        .bind(&data.name)
        .bind(data.verified)
        .fetch_one(&self.pool)
        .await?;
        return Ok(user);
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn get_many(&self, skip: Option<i64>, limit: Option<i64>) -> Result<Vec<User>> {
        let offset = skip.unwrap_or(0);
        let lim = limit.unwrap_or(100);
        let users = query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            OFFSET $1
            LIMIT $2
            "#,
        )
        .bind(offset)
        .bind(lim)
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        query(
            r#"
            DELETE
            FROM users
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
impl UserRepository for PostgresUserRepository {
    async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn update(&self, user: &User) -> Result<User> {
        let user = query_as::<_, User>(
            r#"
            UPDATE users
            SET verified = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(user.verified)
        .bind(user.base.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
