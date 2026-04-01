use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;

use crate::config::Settings;

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let s = Config::builder()
            .add_source(File::with_name("config/base"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .set_default("email.code_expire_min", 15)?
            .build()?;
        s.try_deserialize()
    }
}
