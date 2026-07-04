#![cfg(feature = "ssr")]

use leptos::prelude::ServerFnError;
use std::path::PathBuf;

/// Directory slide images are saved in and served from. Everything in this
/// directory gets served publicly under /screen/slides/images.
pub const SLIDE_IMAGE_DIR: &str = "./slide_images";

pub async fn save_image_bytes(bytes: &[u8], filename: &str, file_type: &str) -> Result<PathBuf, ServerFnError> {
    let mut file_path: PathBuf = [SLIDE_IMAGE_DIR, filename].iter().collect();
    file_path.set_extension(file_type);
    let saved_path = file_path.clone();

    log::info!("Saving image as: {:?}", file_path);
    tokio::fs::write(&file_path, bytes)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(saved_path)
}

pub async fn remove_file(file_path: PathBuf) -> Result<(), ServerFnError> {
    log::info!("Removing file at: {:?}", file_path);
    tokio::fs::remove_file(file_path)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

pub async fn remove_slide_image(id: &str, filetype: &str) -> Result<(), ServerFnError> {
    let mut file_path: PathBuf = [SLIDE_IMAGE_DIR, id].iter().collect();
    file_path.set_extension(filetype);
    remove_file(file_path).await
}
