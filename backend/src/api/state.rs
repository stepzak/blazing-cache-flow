use std::sync::Arc;

use crate::{
    config::Settings, domain::infrastructure::email::EmailSender, services::auth::AuthService,
};

pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub email_sender: Arc<dyn EmailSender>,
    pub settings: Arc<Settings>,
}
