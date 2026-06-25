use crate::core::state::SharedState;

#[tauri::command]
pub async fn send_chat_message(content: String, state: SharedState<'_>) -> Result<(), String> {
    let sender_lock = state.network.ws.lock().await;

    if let Some(_sender) = sender_lock.as_ref() {
        println!("React wants to send: {}", content);
        Ok(())
    } else {
        Err("WebSocket is not connected!".to_string())
    }
}
