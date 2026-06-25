pub mod auth;
pub mod call;
pub mod entity;
pub mod network;
pub mod workspace;

pub use auth::AuthState;
pub use call::CallState;
pub use entity::EntityCache;
pub use network::NetworkState;
pub use workspace::WorkspaceState;

pub struct AppState {
    pub network: NetworkState,
    pub auth: AuthState,
    pub call: CallState,
    pub workspace: WorkspaceState,
    pub cache: EntityCache,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            network: NetworkState::default(),
            auth: AuthState::default(),
            call: CallState::default(),
            workspace: WorkspaceState::default(),
            cache: EntityCache::default(),
        }
    }
}

pub type SharedState<'a> = tauri::State<'a, AppState>;
