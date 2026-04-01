use serde::Deserialize;

pub mod loader;

#[derive(Debug, Deserialize, Clone)]
pub struct EmailSettings {
    pub code_expire_min: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub pool_size: u32,
    pub pool_timeout: u64,
    pub pool_recycle: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSettings {
    pub secret_key: String,
    pub access_expire_min: i64,
    pub refresh_expire_min: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub email: EmailSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}
