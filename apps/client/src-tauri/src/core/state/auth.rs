use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AuthState {
    pub token: Arc<RwLock<Option<String>>>,
    pub user_id: Arc<RwLock<Option<String>>>,
}
