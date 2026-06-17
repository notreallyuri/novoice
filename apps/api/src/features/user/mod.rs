use crate::core::state::SharedState;
use axum::{
    Router,
    routing::{get, post},
};

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/@me", get(controller::get_me))
        .route("/@me/{guild_id}", get(controller::get_guild))
        .route("/@me/g/join", post(controller::join_guild))
}
