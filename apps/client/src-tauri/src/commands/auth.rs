use crate::{core::state::SharedState, error::AppError};
use shared::{
    dtos::{
        auth::{AuthResponse, LoginRequest},
        user::GetMeResponse,
    },
    ws::ClientMessage,
};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn login(
    email: String,
    password: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let res = state
        .network
        .http
        .post(format!("{}/auth/login", crate::API_URL))
        .json(&LoginRequest { email, password })
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    *state.auth.token.write().await = Some(res.token.clone());

    let store = app.store("auth.json")?;
    store.set("token", res.token.clone());
    store.save()?;

    state.network.disconnect_ws().await;
    crate::core::loader::close_loader(app).await?;

    Ok(())
}

#[tauri::command]
pub async fn logout(state: SharedState<'_>, app: AppHandle) -> Result<(), AppError> {
    *state.auth.token.write().await = None;
    *state.auth.user_id.write().await = None;

    let store = app.store("auth.json")?;
    store.delete("token");
    store.save()?;

    state.network.disconnect_ws().await;
    crate::core::loader::open_loader(app).await?;

    Ok(())
}

#[tauri::command]
pub async fn check_auth_status(
    state: SharedState<'_>,
    app_handle: tauri::AppHandle,
) -> Result<bool, AppError> {
    let stored = app_handle.store("auth.json").ok().and_then(|store| {
        let token = store.get("token")?.as_str().map(String::from)?;
        let user_id = store
            .get("user_id")
            .and_then(|v| v.as_str().map(String::from));
        Some((token, user_id))
    });

    if let Some((token, user_id)) = stored {
        *state.auth.token.write().await = Some(token.clone());
        *state.auth.user_id.write().await = user_id;

        state.network.connect_ws(app_handle.clone()).await;
        let _ = state
            .network
            .ws_send(ClientMessage::Identify { token })
            .await;

        let _ = crate::core::loader::close_loader(app_handle).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_initial_data(state: SharedState<'_>) -> Result<GetMeResponse, AppError> {
    let token = state
        .auth
        .token
        .read()
        .await
        .clone()
        .ok_or(AppError::Unauthorized)?;

    let res = state
        .network
        .http
        .get(format!("{}/users/@me", crate::API_URL))
        .bearer_auth(token)
        .send()
        .await?
        .json::<GetMeResponse>()
        .await?;

    Ok(res)
}
