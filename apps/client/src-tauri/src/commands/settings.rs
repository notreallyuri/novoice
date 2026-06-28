use crate::error::AppError;
use shared::data::user_settings::{
    ThemeColor, ThemeDarkMode, ThemeRounding, ThemeSpacing, UISettings,
};
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

fn get_default_ui_settings() -> UISettings {
    UISettings {
        dark_mode: ThemeDarkMode::System,
        rounding: ThemeRounding::Default,
        spacing: ThemeSpacing::Default,
        theme: ThemeColor::Default,
    }
}

#[tauri::command]
pub async fn get_ui_settings(app: AppHandle) -> Result<UISettings, AppError> {
    let store = app.store("settings.json")?;

    if let Some(val) = store.get("ui_settings") {
        if let Ok(settings) = serde_json::from_value::<UISettings>(val) {
            return Ok(settings);
        }
    }

    Ok(get_default_ui_settings())
}

#[tauri::command]
pub async fn update_ui_settings(settings: UISettings, app: AppHandle) -> Result<(), AppError> {
    let store = app.store("settings.json")?;

    let json_val = serde_json::to_value(&settings)?;
    store.set("ui_settings", json_val);
    store.save()?;

    app.emit("ui_settings", &settings)?;

    Ok(())
}
