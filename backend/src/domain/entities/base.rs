use chrono::{self, Utc};
use serde;
use sqlx;

#[derive(sqlx::FromRow, serde::Serialize, Debug, Clone)]
pub struct BaseFields {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl BaseFields {
    pub fn new() -> Self {
        let now = Utc::now();
        BaseFields {
            id: uuid::Uuid::now_v7(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
