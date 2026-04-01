use std::{sync::Arc, time::Duration};

use axum::{Router, routing::{get, post}};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{
        dto::auth::{LoginDTO, RegisterDTO, TokenResponse, UserResponse, ValidateEmailDTO},
        handlers::auth::{login_handler, me_handler, register_handler, verify_email_handler},
        state::AppState,
    },
    config::Settings,
    infrastructure::{
        email::sender::SmtpService,
        repositories::{
            pg_email_code::PostgresEmailCodeRepository, pg_user::PostgresUserRepository,
        },
    },
    services::auth::AuthService,
};

mod api;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod services;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_jwt",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::auth::register_handler,
        crate::api::handlers::auth::verify_email_handler,
        crate::api::handlers::auth::login_handler,
        crate::api::handlers::auth::me_handler
    ),
    components(schemas(RegisterDTO, UserResponse, ValidateEmailDTO, LoginDTO, TokenResponse)),
    modifiers(&SecurityAddon)
)]
struct Docs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Arc::new(Settings::new()?);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(settings.database.pool_size)
        .acquire_timeout(Duration::from_secs(settings.database.pool_timeout))
        .connect(&settings.database.connection_string())
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Database connected and migrated");

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let email_repo = Arc::new(PostgresEmailCodeRepository::new(pool.clone()));
    let email_sender = Arc::new(SmtpService::new((*settings).clone())?);
    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        email_repo.clone(),
        &settings,
    ));
    let app_state = Arc::new(AppState {
        auth_service,
        email_sender: email_sender.clone(),
        settings: settings.clone(),
    });

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", Docs::openapi()))
        .route("/api/auth/verify_email", post(verify_email_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/me", get(me_handler))
        .with_state(app_state);

    let addr = "127.0.0.1:5000";
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server started at http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
