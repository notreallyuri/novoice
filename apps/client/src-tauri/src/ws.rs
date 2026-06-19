use futures_util::{SinkExt, StreamExt};
use shared::ws::{ClientMessage, ServerMessage};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const URL: &str = "ws://127.0.0.1:3333/ws";

pub async fn start_ws(app_handle: AppHandle, mut rx: mpsc::UnboundedReceiver<ClientMessage>) {
    loop {
        let (ws_stream, _) = match connect_async(URL).await {
            Ok(stream) => stream,
            Err(e) => {
                tracing::error!("Failed to connect to WebSocket: {}. Retrying in 3s...", e);
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                continue;
            }
        };

        tracing::info!("Connected to NoVoice API");
        let (mut write, mut read) = ws_stream.split();

        loop {
            tokio::select! {
                Some(msg_res) = read.next() => {
                    match msg_res {
                        Ok(Message::Text(text)) => {
                            if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                                let _ = app_handle.emit("server_event", server_msg);
                            } else {
                                tracing::warn!("Failed to parse Server Message: {}", text);
                            }
                        }
                        Ok(Message::Close(_)) | Err(_) => {
                            tracing::warn!("WebSocket connection closed by server.");
                            break;
                        }
                        _ => {}
                    }
                },
                Some(client_msg) = rx.recv() => {
                    if let Ok(json) = serde_json::to_string(&client_msg) {
                        if write.send(Message::Text(json.into())).await.is_err() {
                            tracing::error!("Failed to send message to server, socket dead.");
                            break;
                        }
                    }
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
