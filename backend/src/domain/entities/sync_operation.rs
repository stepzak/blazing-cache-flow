use uuid::Uuid;

use super::base::BaseFields;

#[derive(Debug, Clone, Copy, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "tabletype", rename_all = "lowercase")]
pub enum TableType {
    Categories,
    Account,
    Operations,
}

#[derive(Debug, Clone, Copy, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "action", rename_all = "lowercase")]
pub enum Action {
    Create,
    Update,
    Delete,
}

#[derive(sqlx::FromRow)]
pub struct SyncOperation {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub processing_id: Uuid,
    pub table_type: TableType,
    pub action: Action,
}

impl SyncOperation {
    pub fn new(processing_id: Uuid, table_type: TableType, action: Action) -> Self {
        SyncOperation {
            base: BaseFields::new(),
            processing_id,
            table_type,
            action,
        }
    }
}
