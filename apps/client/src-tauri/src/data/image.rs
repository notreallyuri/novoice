use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CropData {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale_x: f64,
    pub scale_y: f64,
}

pub struct TempCroppedImage {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateImageRequest {
    pub path: Option<String>,
    pub crop: Option<CropData>,
}
