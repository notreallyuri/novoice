use crate::{core::state::SharedState, error::AppError};
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn start_call(
    state: SharedState<'_>,
    channel_id: String,
    channel_name: String,
    app: AppHandle,
) -> Result<(), AppError> {
    let call_state = {
        let mut call_state = state.call.inner.write().await;
        call_state.is_active = true;
        call_state.channel_id = Some(channel_id);
        call_state.channel_name = Some(channel_name);

        call_state.clone()
    };

    app.emit("call_state", call_state)?;

    Ok(())
}

#[tauri::command]
pub async fn close_call(state: SharedState<'_>, app: AppHandle) -> Result<(), AppError> {
    let call_state = {
        let mut call_state = state.call.inner.write().await;
        call_state.is_active = false;
        call_state.channel_id = None;
        call_state.channel_name = None;

        call_state.clone()
    };

    app.emit("call_state", call_state)?;

    Ok(())
}

#[tauri::command]
pub async fn toggle_mute(state: SharedState<'_>, app: AppHandle) -> Result<(), AppError> {
    let call_state = {
        let mut call_state = state.call.inner.write().await;
        call_state.is_muted = !call_state.is_muted;
        // TODO: Call actual WebRTC hardware mute logic here
        call_state.clone()
    };

    app.emit("call_state", call_state)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_deafen(state: SharedState<'_>, app: AppHandle) -> Result<(), AppError> {
    let call_state = {
        let mut call_state = state.call.inner.write().await;
        call_state.is_deafened = !call_state.is_deafened;
        // TODO: Call actual WebRTC hardware deafen logic here
        call_state.clone()
    };

    app.emit("call_state", call_state)?;
    Ok(())
}
