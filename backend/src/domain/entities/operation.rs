use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use super::base::BaseFields;

#[derive(sqlx::FromRow)]
pub struct Operation {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub account_uuid: Uuid,
    pub transfer_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub amount: Decimal,
    pub date: DateTime<Utc>,
    pub notes: String,
}

impl Operation {
    pub fn new(
        account_uuid: Uuid,
        transfer_id: Option<Uuid>,
        category_id: Option<Uuid>,
        amount: Decimal,
        date: DateTime<Utc>,
        notes: String,
    ) -> Self {
        Operation {
            base: BaseFields::new(),
            account_uuid,
            transfer_id,
            category_id,
            amount,
            date,
            notes,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct Transfer {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub account_from_id: Uuid,
    pub account_to_id: Uuid,
}

impl Transfer {
    pub fn new(account_from_id: Uuid, account_to_id: Uuid) -> Self {
        Transfer {
            base: BaseFields::new(),
            account_from_id,
            account_to_id,
        }
    }
}
