pub const WS_URL: &str = "ws://localhost:3333/ws";
pub const API_URL: &str = "http://localhost:3333/api";

pub mod commands;
pub mod core;
pub mod data;
pub mod error;
pub mod ws;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt().init();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(core::state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            workspace::close_space,
            workspace::open_space,
            workspace::request_initial_spaces,
            workspace::set_layout_direction,
            workspace::split_space,
            workspace::replace_space,
            call::start_call,
            call::close_call,
            call::toggle_mute,
            call::toggle_deafen,
            auth::login,
            auth::logout,
            auth::get_initial_data,
            chat::send_chat_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
