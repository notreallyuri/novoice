use reqwest::Client;
use shared::ws::ClientMessage;
use std::{sync::Arc, time::Duration};
use tauri::AppHandle;
use tokio::sync::{mpsc, Mutex};

use crate::error::AppError;

pub struct NetworkState {
    pub ws: Arc<Mutex<Option<mpsc::UnboundedSender<ClientMessage>>>>,
    pub http: Client,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            ws: Arc::new(Mutex::new(None)),
            http: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }
}

impl NetworkState {
    pub async fn connect_ws(&self, app: AppHandle) {
        let already_connected = self.ws.lock().await.is_some();

        if already_connected {
            return;
        }

        let (tx, rx) = mpsc::unbounded_channel::<ClientMessage>();

        *self.ws.lock().await = Some(tx);

        tokio::spawn(crate::ws::start_ws(app, rx));
    }

    pub async fn disconnect_ws(&self) {
        *self.ws.lock().await = None;
    }

    pub async fn ws_send(&self, message: ClientMessage) -> Result<(), AppError> {
        let tx = self.ws.lock().await;

        match tx.as_ref() {
            Some(tx) => tx
                .send(message)
                .map_err(|e| AppError::Internal(format!("Failed to send WS message: {e}"))),
            None => Err(AppError::Internal("WebSocket not connected".into())),
        }
    }
}
