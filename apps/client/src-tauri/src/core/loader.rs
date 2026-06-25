use crate::error::AppError;
use tauri::{AppHandle, Manager};

pub async fn close_loader(app_handle: AppHandle) -> Result<(), AppError> {
    if let Some(loader_window) = app_handle.get_webview_window("loader") {
        loader_window.close()?;
    }

    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window.show()?;
        main_window.set_focus()?;
    }

    Ok(())
}

pub async fn open_loader(app_handle: AppHandle) -> Result<(), AppError> {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window.close()?;
    }

    if let Some(loader_window) = app_handle.get_webview_window("loader") {
        loader_window.show()?;
        loader_window.set_focus()?;
    }

    Ok(())
}
