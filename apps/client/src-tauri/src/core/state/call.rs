use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallStateData {
    pub is_active: bool,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub is_muted: bool,
    pub is_deafened: bool,
}

impl Default for CallStateData {
    fn default() -> Self {
        Self {
            is_active: false,
            channel_id: None,
            channel_name: None,
            is_muted: false,
            is_deafened: false,
        }
    }
}

pub struct CallState {
    pub inner: Arc<RwLock<CallStateData>>,
}

impl Default for CallState {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CallStateData::default())),
        }
    }
}
