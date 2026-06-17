use crate::{core::state::SharedState, features};
use axum::{Router, routing::get};

pub fn router(_: &SharedState) -> Router<SharedState> {
    Router::new()
        .route("/health", get(super::health::health_check))
        .nest("/users", features::user::router())
        .nest("/users/@me/relationships", features::relationship::router())
        .nest("/users/@me/dms", features::dms::router())
        .nest("/auth", features::auth::router())
        .nest("/guilds", features::guild::router())
        .nest(
            "/guilds/{guild_id}/categories",
            features::category::router(),
        )
        .nest("/guilds/{guild_id}/channels", features::channel::router())
        .nest(
            "/guilds/{guild_id}/members",
            features::guild_member::router(),
        )
        .nest("/messages/{channel_id}", features::messages::router())
}
