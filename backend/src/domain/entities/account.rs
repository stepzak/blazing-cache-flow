use rust_decimal::Decimal;
use uuid::Uuid;

use super::base::BaseFields;

#[derive(sqlx::FromRow)]
pub struct Account {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub name: String,
    pub user_id: Uuid,
    pub funds: Decimal,
    pub color: String,
}

impl Account {
    pub fn new(name: &str, user_id: Uuid, funds: Decimal, color: &str) -> Self {
        Account {
            base: BaseFields::new(),
            user_id,
            name: name.into(),
            funds,
            color: color.into(),
        }
    }
}
