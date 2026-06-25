use crate::core::state::AppState;

pub const WS_URL: &str = "ws://localhost:3333/ws";
pub const API_URL: &str = "http://localhost:3333/api";

pub mod commands;
pub mod core;
pub mod data;
pub mod error;
pub mod ws;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::chat::send_chat_message,
            commands::workspace::close_space,
            commands::workspace::open_space,
            commands::workspace::request_initial_spaces,
            commands::workspace::set_layout_direction,
            commands::workspace::split_space,
            commands::workspace::replace_space,
            commands::call::start_call,
            commands::call::close_call,
            commands::call::toggle_mute,
            commands::call::toggle_deafen,
            commands::auth::login,
            commands::auth::logout,
            commands::auth::check_auth_status,
            commands::auth::get_initial_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
