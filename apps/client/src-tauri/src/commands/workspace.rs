use crate::{
    core::state::{
        workspace::{SpaceData, WorkspaceNode},
        SharedState,
    },
    error::AppError,
};
use tauri::{AppHandle, Emitter};

const SPACES_STATE: &str = "spaces_state";

#[tauri::command]
pub async fn request_initial_spaces(
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = state.workspace.inner.read().await;
    app.emit(SPACES_STATE, data.clone())?;
    Ok(())
}

#[tauri::command]
pub async fn split_space(
    target_id: String,
    new_group_id: String,
    new_panel_id: String,
    target_panel_id: String,
    title: String,
    space_type: String,
    split_direction: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = {
        let mut ws = state.workspace.inner.write().await;

        let new_panel = WorkspaceNode::Panel {
            data: SpaceData {
                id: new_panel_id,
                target_id: target_panel_id,
                title,
                space_type,
            },
        };

        ws.root
            .split_node(&target_id, new_panel, new_group_id, split_direction);
        ws.clone()
    };

    app.emit(SPACES_STATE, data)?;
    Ok(())
}

#[tauri::command]
pub async fn replace_space(
    target_id: String,
    new_panel_id: String,
    target_panel_id: String,
    title: String,
    space_type: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = {
        let mut ws = state.workspace.inner.write().await;
        ws.root.replace_node(
            &target_id,
            SpaceData {
                id: new_panel_id,
                target_id: target_panel_id,
                title,
                space_type,
            },
        );
        ws.clone()
    };
    app.emit(SPACES_STATE, data)?;
    Ok(())
}

#[tauri::command]
pub async fn open_space(
    id: String,
    target_id: String,
    title: String,
    space_type: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = {
        let mut ws = state.workspace.inner.write().await;

        let new_panel = WorkspaceNode::Panel {
            data: SpaceData {
                id,
                target_id,
                title,
                space_type,
            },
        };

        ws.root.add_to_root(new_panel);
        ws.clone()
    };

    app.emit(SPACES_STATE, data)?;
    Ok(())
}

#[tauri::command]
pub async fn close_space(
    id: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = {
        let mut ws = state.workspace.inner.write().await;
        ws.root.remove_node(&id);
        ws.clone()
    };
    app.emit(SPACES_STATE, data.clone())?;
    Ok(())
}

#[tauri::command]
pub async fn set_layout_direction(
    direction: String,
    state: SharedState<'_>,
    app: AppHandle,
) -> Result<(), AppError> {
    let data = {
        let mut ws = state.workspace.inner.write().await;
        ws.root.set_root_direction(direction);
        ws.clone()
    };

    app.emit(SPACES_STATE, data)?;
    Ok(())
}
