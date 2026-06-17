use crate::core::state::SharedState;
use chrono::{Duration, Utc};
use sea_orm::ConnectionTrait;

pub async fn start_daily_sweeper(state: SharedState) {
    tokio::spawn(async move {
        tracing::info!("Daily sweeper worker started.");

        loop {
            let now = Utc::now();

            let mut next_run = now
                .date_naive()
                .and_hms_opt(3, 0, 0)
                .expect("Valid time")
                .and_utc();

            if now >= next_run {
                next_run += Duration::days(1);
            }

            let sleep_duration = (next_run - now)
                .to_std()
                .expect("Next run should be in the future");

            tracing::info!(
                "Sweeper sleeping for {} hours until next run.",
                sleep_duration.as_secs() / 3600
            );

            tokio::time::sleep(sleep_duration).await;

            tracing::info!("Waking up for daily garbage collection...");

            let mut conn = match state.redis.cache.get().await {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Sweeper failed to get Redis connection: {}", e);
                    continue;
                }
            };

            let acquired_lock: Option<String> = deadpool_redis::redis::cmd("SET")
                .arg("lock:daily_sweeper")
                .arg("locked")
                .arg("NX")
                .arg("EX")
                .arg(300)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            if acquired_lock.is_none() {
                tracing::info!("Another instance is already running the sweeper. Skipping.");
                continue;
            }

            match state
                .db
                .execute_unprepared("DELETE FROM guild_bans WHERE expires_at < NOW()")
                .await
            {
                Ok(res) => tracing::info!("Swept {} expired bans.", res.rows_affected()),
                Err(e) => tracing::error!("Failed to sweep bans: {}", e),
            }
        }
    });
}
