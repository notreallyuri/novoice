use crate::{
    core::{
        error::AppError,
        state::{SessionId, SharedState},
    },
    features::ws::services::identity::{handle_connect, handle_disconnect},
};
use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use redis::AsyncCommands;
use shared::{
    data::UserId,
    ws::{ClientMessage, ServerMessage},
};
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| {
        if let Err(e) = socket_handler(socket, state).await {
            tracing::error!("WebSocket dropped: {e}");
        }
    })
}

pub async fn socket_handler(socket: WebSocket, state: SharedState) -> Result<(), AppError> {
    let session_id = SessionId(Uuid::new_v4());

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let current_user_id = loop {
        match ws_receiver.next().await {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(ClientMessage::Identify { token }) => {
                    let mut conn = state.redis.sessions.get().await?;
                    let redis_key = format!("session:{token}");

                    let user_id_result: Option<String> = conn.get(&redis_key).await.unwrap_or(None);

                    match user_id_result {
                        Some(raw_id) => {
                            let user_id = UserId(Uuid::parse_str(&raw_id)?);
                            handle_connect(&state, user_id, session_id, tx.clone()).await?;
                            break user_id;
                        }
                        None => {
                            send_error(&tx, "Invalid or expired session token");
                            send_task.abort();
                            return Ok(());
                        }
                    }
                }
                _ => {
                    send_error(&tx, "Must Identify first");
                }
            },
            Some(Ok(Message::Ping(data))) => {
                let _ = tx.send(Message::Pong(data));
            }
            _ => {
                send_task.abort();
                return Ok(());
            }
        }
    };

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    match client_msg {
                        ClientMessage::SetPresence { presence } => {
                            // TODO: Handle manual status updates
                            tracing::debug!(
                                "User {} requested presence update: {:?}",
                                current_user_id.0,
                                presence
                            );
                        }
                        ClientMessage::Rtc(_rtc_event) => {
                            // TODO: Handle voice/video signaling
                        }
                        _ => {}
                    }
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = tx.send(Message::Pong(data));
            }
            Ok(Message::Close(_)) | Err(_) => {
                break;
            }
            _ => {}
        }
    }

    handle_disconnect(&state, current_user_id, session_id).await;
    send_task.abort();

    Ok(())
}

fn send_error(tx: &tokio::sync::mpsc::UnboundedSender<Message>, msg: &str) {
    let payload = ServerMessage::Error {
        message: msg.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&payload) {
        let _ = tx.send(Message::Text(json.into()));
    }
}
