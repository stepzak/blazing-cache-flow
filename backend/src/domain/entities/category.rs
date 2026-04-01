use uuid::Uuid;

use super::base::BaseFields;

#[derive(sqlx::FromRow)]
pub struct Category {
    #[sqlx(flatten)]
    pub base: BaseFields,
    pub name: String,
    pub user_id: Uuid,
}

impl Category {
    pub fn new(name: &str, user_id: Uuid) -> Self {
        Category {
            base: BaseFields::new(),
            name: name.into(),
            user_id,
        }
    }
}
