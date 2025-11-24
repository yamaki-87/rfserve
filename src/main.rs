pub mod config;
pub mod di;
pub mod domain;
pub mod infra;
pub mod service;
use anyhow::Result;
use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

use crate::{di::AppState, infra::server};
#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let envs = config::envs::get_instance();
    log_init();
    let db = infra::db::db_init().await?;

    let app_state = AppState::new(db);
    let app = server::get_route(app_state).await;

    let listener = tokio::net::TcpListener::bind(envs.get_addr()).await?;
    tracing::info!("server listen:{}", envs.get_addr());
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("axum serve error:{}", e);
    }
    Ok(())
}

fn log_init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
