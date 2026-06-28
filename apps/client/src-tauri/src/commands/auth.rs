use crate::{core::state::SharedState, error::AppError};
use shared::dtos::{
    auth::{AuthResponse, LoginRequest},
    user::GetMeResponse,
};
use shared::ws::ClientMessage;
use tauri::{AppHandle, Emitter, Manager};
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
    state.network.connect_ws(app.clone()).await;

    crate::core::loader::transition_auth_to_main(app).await?;

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

    if let Some(main_window) = app.get_webview_window("main") {
        main_window.hide()?;
    }
    if let Some(auth_window) = app.get_webview_window("auth") {
        auth_window.show()?;
        auth_window.set_focus()?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_initial_data(
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<GetMeResponse, AppError> {
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

    *state.cache.current_user.write().await = Some(res.user.clone());

    let _ = app.emit("ui_settings", &res.user.settings.ui);

    Ok(res)
}

#[tauri::command]
pub async fn check_auth_status(state: SharedState<'_>, app: AppHandle) -> Result<(), AppError> {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let stored = app.store("auth.json").ok().and_then(|store| {
        let token = store.get("token")?.as_str().map(String::from)?;
        let user_id = store
            .get("user_id")
            .and_then(|v| v.as_str().map(String::from));
        Some((token, user_id))
    });

    if let Some((token, user_id)) = stored {
        *state.auth.token.write().await = Some(token.clone());
        *state.auth.user_id.write().await = user_id;

        state.network.connect_ws(app.clone()).await;
        let _ = state
            .network
            .ws_send(ClientMessage::Identify { token })
            .await;

        crate::core::loader::transition_loader_to_main(app).await?;
    } else {
        crate::core::loader::transition_loader_to_auth(app).await?;
    }

    Ok(())
}
