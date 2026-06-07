use crate::core::state::SharedState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use deadpool_redis::redis::cmd;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;

pub async fn health_check(State(state): State<SharedState>) -> impl IntoResponse {
    let timeout_duration = Duration::from_secs(2);

    let (db_res, scylla_res, sessions_res, presence_res, message_res) = tokio::join!(
        timeout(timeout_duration, state.db.ping()),
        timeout(
            timeout_duration,
            state
                .scylla
                .session
                .query_unpaged("SELECT cluster_name FROM system.local", &[])
        ),
        timeout(timeout_duration, async {
            let mut conn = state.redis.sessions.get().await.map_err(|_| "pool_error")?;
            let _pong: String = cmd("PING")
                .query_async(&mut conn)
                .await
                .map_err(|_| "ping_error")?;
            Ok::<_, &'static str>(())
        }),
        timeout(timeout_duration, async {
            let mut conn = state.redis.presence.get().await.map_err(|_| "pool_error")?;
            let _pong: String = cmd("PING")
                .query_async(&mut conn)
                .await
                .map_err(|_| "ping_error")?;
            Ok::<_, &'static str>(())
        }),
        timeout(timeout_duration, async {
            let mut conn = state.redis.messages.get().await.map_err(|_| "pool_error")?;
            let _pong: String = cmd("PING")
                .query_async(&mut conn)
                .await
                .map_err(|_| "ping_error")?;
            Ok::<_, &'static str>(())
        }),
    );

    let db_up = matches!(db_res, Ok(Ok(_)));
    let scylla_up = matches!(scylla_res, Ok(Ok(_)));
    let redis_sessions_up = matches!(sessions_res, Ok(Ok(_)));
    let redis_presence_up = matches!(presence_res, Ok(Ok(_)));
    let redis_messages_up = matches!(message_res, Ok(Ok(_)));

    let all_healthy =
        db_up && scylla_up && redis_sessions_up && redis_presence_up && redis_messages_up;

    let status_code = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let body = json!({
        "status": if all_healthy { "healthy" } else { "unhealthy" },
        "database": if db_up { "connected" } else { "disconnected" },
        "scylla": if scylla_up { "connected" } else { "disconnected" },
        "redis_sessions": if redis_sessions_up { "connected" } else { "disconnected" },
        "redis_presence": if redis_presence_up { "connected" } else { "disconnected" },
        "redis_messages": if redis_messages_up { "connected" } else { "disconnected" }
    });

    (status_code, Json(body))
}
