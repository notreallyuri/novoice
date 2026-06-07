use crate::{core::state::SharedState, features};
use axum::{Router, routing::get};

pub fn router(_state: &SharedState) -> Router<SharedState> {
    Router::new()
        .route("/health", get(super::health::health_check))
        .nest("/auth", features::auth::router())
        .nest("/guilds", features::guild::router())
        .nest("/messages/{channel_id}", features::messages::router())
}
