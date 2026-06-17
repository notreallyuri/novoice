use super::error::AppError;
use crate::features::ws;

use axum::{Router, routing::get};
use axum_client_ip::ClientIpSource;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = std::sync::Arc::new(super::state::AppState::load().await?);

    ws::listener::redis_listener(state.clone()).await;
    super::workers::start_daily_sweeper(state.clone()).await;

    let app = Router::new()
        .route("/ws", get(ws::handler::ws_handler))
        .nest("/api", super::router::router(&state))
        .with_state(state)
        .layer(ClientIpSource::ConnectInfo.into_extension());

    let addr = "0.0.0.0:3333";
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to bind to address: {}", e)))?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        tracing::info!("Shutting down gracefully...")
    })
    .await
    .map_err(|e| AppError::Internal(format!("Server crashed: {e}")))
}
