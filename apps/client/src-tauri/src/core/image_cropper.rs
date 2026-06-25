use crate::{
    data::image::{CropData, TempCroppedImage},
    error::AppError,
};
use image::{
    codecs::gif::{GifDecoder, GifEncoder},
    AnimationDecoder,
};
use std::{
    fs,
    io::BufReader,
    time::{SystemTime, UNIX_EPOCH},
};

impl TempCroppedImage {
    pub fn process(path: &str, crop: &CropData) -> Result<Self, AppError> {
        let mut img = image::open(path)
            .map_err(|e| AppError::Internal(format!("Failed to open original image: {e}")))?;

        let x = (crop.x * crop.scale_x).max(0.0) as u32;
        let y = (crop.y * crop.scale_y).max(0.0) as u32;
        let width = (crop.width * crop.scale_x).max(1.0) as u32;
        let height = (crop.height * crop.scale_y).max(1.0) as u32;

        let cropped_img = image::imageops::crop(&mut img, x, y, width, height).to_image();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let actual_format = image::ImageFormat::from_path(path).ok().unwrap_or_else(|| {
            let bytes = fs::read(path).unwrap_or_default();
            image::guess_format(&bytes).unwrap_or(image::ImageFormat::Png)
        });

        let ext = match actual_format {
            image::ImageFormat::Png => "png",
            image::ImageFormat::Gif => "gif",
            image::ImageFormat::WebP => "webp",
            image::ImageFormat::Jpeg => "jpg",
            _ => "png",
        };

        if ext == "gif" {
            return Self::process_gif(path, crop);
        }

        let final_img: image::DynamicImage = if ext == "jpg" || ext == "jpeg" {
            image::DynamicImage::ImageRgba8(cropped_img)
                .into_rgb8()
                .into()
        } else {
            image::DynamicImage::ImageRgba8(cropped_img)
        };

        let temp_path = std::env::temp_dir().join(format!("temp_{}.{}", timestamp, ext));

        final_img
            .save(&temp_path)
            .map_err(|e| AppError::Internal(format!("Failed to save cropped image: {e}")))?;

        Ok(Self { path: temp_path })
    }

    fn process_gif(path: &str, crop: &CropData) -> Result<Self, AppError> {
        let x = (crop.x * crop.scale_x).max(0.0) as u32;
        let y = (crop.y * crop.scale_y).max(0.0) as u32;
        let width = (crop.width * crop.scale_x).max(1.0) as u32;
        let height = (crop.height * crop.scale_y).max(1.0) as u32;

        let input = BufReader::new(
            fs::File::open(path)
                .map_err(|e| AppError::Internal(format!("Failed to open GIF: {e}")))?,
        );
        let decoder = GifDecoder::new(input)
            .map_err(|e| AppError::Internal(format!("Failed to decode GIF: {e}")))?;
        let frames = decoder
            .into_frames()
            .collect_frames()
            .map_err(|e| AppError::Internal(format!("Failed to collect GIF frames: {e}")))?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let temp_path = std::env::temp_dir().join(format!("temp_{}.gif", timestamp));
        let output = fs::File::create(&temp_path)
            .map_err(|e| AppError::Internal(format!("Failed to create temp GIF: {e}")))?;

        let mut encoder = GifEncoder::new(output);
        encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .map_err(|e| AppError::Internal(format!("Failed to set GIF repeat: {e}")))?;

        for frame in frames {
            let delay = frame.delay();
            let img = frame.into_buffer();
            let cropped = image::imageops::crop_imm(&img, x, y, width, height).to_image();
            let new_frame = image::Frame::from_parts(cropped, 0, 0, delay);
            encoder
                .encode_frame(new_frame)
                .map_err(|e| AppError::Internal(format!("Failed to encode GIF frame: {e}")))?;
        }

        Ok(Self { path: temp_path })
    }
}

impl Drop for TempCroppedImage {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}
