use crate::error::AppError;
use tauri::{AppHandle, Manager};

pub async fn transition_loader_to_auth(app_handle: AppHandle) -> Result<(), AppError> {
    if let Some(auth_window) = app_handle.get_webview_window("auth") {
        auth_window.show()?;
        auth_window.set_focus()?;
    }
    if let Some(loader_window) = app_handle.get_webview_window("loader") {
        loader_window.close()?;
    }
    Ok(())
}

pub async fn transition_loader_to_main(app_handle: AppHandle) -> Result<(), AppError> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window.show()?;
        main_window.set_focus()?;
    }
    if let Some(loader_window) = app_handle.get_webview_window("loader") {
        loader_window.close()?;
    }
    if let Some(auth_window) = app_handle.get_webview_window("auth") {
        auth_window.close()?;
    }
    Ok(())
}

pub async fn transition_auth_to_main(app_handle: AppHandle) -> Result<(), AppError> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window.show()?;
        main_window.set_focus()?;
    }
    if let Some(auth_window) = app_handle.get_webview_window("auth") {
        auth_window.close()?;
    }
    Ok(())
}
