use super::Train;
use crate::sarif::{watch, wreck};
use anyhow::anyhow;
use std::fs;

/// Downloads the OEWL from the specified URL and saves it to the local path.
///
/// # Arguments
/// * `train` - A `Train` struct containing the URL and local path for OEWL.
///
/// # Returns
/// The modified `Train` object.
pub fn download_oewl(mut train: Train) -> Train {
    train.watch.message = format!("Downloading OEWL from: {}", train.oewl_url);

    let response_result = reqwest::blocking::get(&train.oewl_url);
    if let Err(e) = response_result {
        train.wreck.message = format!("Failed to download OEWL from {}: {}", train.oewl_url, e);
        return train;
    }
    let response = response_result.unwrap();

    let status_check_result = response.error_for_status();
    if let Err(e) = status_check_result {
        train.wreck.message = format!("Server returned an error for {}: {}", train.oewl_url, e);
        return train;
    }
    let valid_response = status_check_result.unwrap();

    let content_result = valid_response.text();
    if let Err(e) = content_result {
        train.wreck.message = format!("Failed to read response text: {}", e);
        return train;
    }
    let content = content_result.unwrap();

    let parent_dir = train.oewl_path.parent().map(|p| p.to_path_buf());
    if let Some(parent) = parent_dir {
        if let Err(e) = fs::create_dir_all(&parent) {
            train.wreck.message = format!("Failed to create directory {}: {}", parent.display(), e);
            return train;
        }
    }

    if let Err(e) = fs::write(&train.oewl_path, &content) {
        train.wreck.message = format!("Failed to write OEWL to {}: {}", train.oewl_path.display(), e);
        return train;
    }

    train.oewl_content = Some(content);
    train.watch.message = format!(
        "Successfully downloaded and saved OEWL to: {}",
        train.oewl_path.display()
    );

    train
}