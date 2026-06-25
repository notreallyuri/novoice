use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Deserialize)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    #[error("WebRTC hardware/connection error: {0}")]
    WebRtc(String),
    #[error("State management error: {0}")]
    State(String),
    #[error("Internal client error: {0}")]
    Internal(String),
    #[error("Unauthorized")]
    Unauthorized,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        tracing::error!("HTTP Request failed: {}", err);
        Self::Network(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("JSON Serialization failed: {}", err);
        Self::Internal(err.to_string())
    }
}

impl From<webrtc::Error> for AppError {
    fn from(err: webrtc::Error) -> Self {
        tracing::error!("WebRTC Engine Error: {}", err);
        Self::WebRtc("A real-time communication error occurred".into())
    }
}

impl From<tauri::Error> for AppError {
    fn from(err: tauri::Error) -> Self {
        tracing::error!("Tauri IPC Error: {}", err);
        Self::Internal("Failed to communicate with the application window".into())
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        tracing::error!("Image processing error: {}", err);
        Self::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("IO error: {}", err);
        Self::Internal(err.to_string())
    }
}

impl From<tauri_plugin_store::Error> for AppError {
    fn from(err: tauri_plugin_store::Error) -> Self {
        tracing::error!("Store error: {}", err);
        Self::Internal(err.to_string())
    }
}
