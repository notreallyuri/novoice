use crate::state::{AppState, ViewPanel};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;

pub mod state;
pub mod ws;

#[tauri::command]
async fn send_chat_message(
    content: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let sender_lock = state.ws.lock().await;

    if let Some(_sender) = sender_lock.as_ref() {
        println!("React wants to send: {}", content);
        Ok(())
    } else {
        Err("WebSocket is not connected!".to_string())
    }
}

#[tauri::command]
fn get_active_panels(state: tauri::State<'_, AppState>) -> Result<Vec<ViewPanel>, String> {
    let panels = state.cache.current_panels.read().unwrap();
    Ok(panels.clone())
}

#[tauri::command]
fn request_initial_panels(state: tauri::State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    let panels = state.cache.current_panels.read().unwrap();
    app.emit("panels_updated", panels.clone()).unwrap();
    Ok(())
}

#[tauri::command]
fn open_panel(
    panel_id: String,
    target_id: String,
    title: String,
    panel_type: String,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let mut panels = state.cache.current_panels.write().unwrap();

    panels.push(ViewPanel {
        id: panel_id,
        target_id,
        title,
        panel_type,
    });

    app.emit("panels_updated", panels.clone()).unwrap();

    Ok(())
}

#[tauri::command]
fn close_panel(
    id_to_remove: String,
    state: tauri::State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let mut panels = state.cache.current_panels.write().unwrap();
    panels.retain(|p| p.id != id_to_remove);
    app.emit("panels_updated", panels.clone()).unwrap();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            send_chat_message,
            get_active_panels,
            close_panel,
            request_initial_panels,
            open_panel
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<AppState>();

            let (tx, rx) = mpsc::unbounded_channel();

            tauri::async_runtime::block_on(async {
                *state.ws.lock().await = Some(tx);
            });

            tauri::async_runtime::spawn(async move {
                ws::start_ws(app_handle, rx).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
