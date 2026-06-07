use crate::core::{error::AppError, statements::ScyllaStatements};
use axum::extract::ws::Message;
use dashmap::DashMap;
use shared::data::UserId;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{sync::mpsc, time::timeout};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SessionId(pub uuid::Uuid);
pub type Tx = mpsc::UnboundedSender<Message>;

pub struct WsSession {
    pub tx: Tx,
    pub user_id: UserId,
}

struct S3Credentials {
    pub endpoint_url: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
}

struct AppCredentials {
    db_url: String,
    scylla_url: String,
    redis_presence_url: String,
    redis_sessions_url: String,
    redis_messages_url: String,
    redis_cache_url: String,
}

pub struct S3 {
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
}

pub struct Redis {
    pub presence: deadpool_redis::Pool,
    pub sessions: deadpool_redis::Pool,
    pub messages: deadpool_redis::Pool,
    pub cache: deadpool_redis::Pool,
}

pub struct Scylla {
    pub session: scylla::client::session::Session,
    pub statements: ScyllaStatements,
}

pub struct AppState {
    pub s3: S3,
    pub db: sea_orm::DatabaseConnection,
    pub scylla: Scylla,
    pub redis: Redis,
    pub active_sessions: DashMap<UserId, Arc<RwLock<HashMap<SessionId, WsSession>>>>,
}

impl AppCredentials {
    fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            db_url: get_env("DATABASE_URL")?,
            scylla_url: get_env("SCYLLA_URL")?,
            redis_presence_url: get_env("REDIS_PRESENCE_URL")?,
            redis_sessions_url: get_env("REDIS_SESSIONS_URL")?,
            redis_messages_url: get_env("REDIS_MESSAGES_URL")?,
            redis_cache_url: get_env("REDIS_CACHE_URL")?,
        })
    }
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub async fn load() -> Result<Self, AppError> {
        let credentials = AppCredentials::from_env()?;
        let s3 = S3::from_env()?;

        let scylla_builder = scylla::client::session_builder::SessionBuilder::new()
            .known_node(&credentials.scylla_url)
            .use_keyspace("chat", false);

        let (db_res, scylla_res) = tokio::join!(
            timeout(
                Duration::from_secs(5),
                sea_orm::Database::connect(&credentials.db_url)
            ),
            timeout(Duration::from_secs(5), scylla_builder.build())
        );

        let db = db_res.map_err(|_| AppError::Internal("Database connection timeout".into()))??;
        let scylla =
            scylla_res.map_err(|_| AppError::Internal("Scylla connection timeout".into()))??;
        let statements = ScyllaStatements::prepare(&scylla).await?;

        let presence = deadpool_redis::Config::from_url(credentials.redis_presence_url)
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        let sessions = deadpool_redis::Config::from_url(credentials.redis_sessions_url)
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        let messages = deadpool_redis::Config::from_url(credentials.redis_messages_url)
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        let cache = deadpool_redis::Config::from_url(credentials.redis_cache_url)
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        Ok(Self {
            s3,
            db,
            scylla: Scylla {
                session: scylla,
                statements,
            },
            redis: Redis {
                presence,
                sessions,
                messages,
                cache,
            },
            active_sessions: DashMap::new(),
        })
    }
}

impl S3 {
    fn from_env() -> Result<Self, AppError> {
        let s3_config = S3Credentials {
            endpoint_url: get_env("R2_ENDPOINT_URL")?,
            access_key_id: get_env("R2_ACCESS_KEY_ID")?,
            secret_access_key: get_env("R2_SECRET_ACCESS_KEY")?,
            bucket: get_env("R2_BUCKET")?,
        };

        let credentials = aws_sdk_s3::config::Credentials::new(
            &s3_config.access_key_id,
            s3_config.secret_access_key,
            None,
            None,
            "static",
        );

        let config = aws_sdk_s3::config::Builder::new()
            .behavior_version_latest()
            .endpoint_url(s3_config.endpoint_url)
            .region(aws_sdk_s3::config::Region::new("auto"))
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        Ok(Self {
            client: aws_sdk_s3::Client::from_conf(config),
            bucket: s3_config.bucket,
        })
    }
}

fn get_env(key: &str) -> Result<String, AppError> {
    std::env::var(key).map_err(|_| AppError::Internal(format!("{key} must be set")))
}
