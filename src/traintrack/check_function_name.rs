use crate::state::Train;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn log_sarif_json(train: &mut Train) {
    // Ensure app_logs_dir is set
    let dir_path_owned = match train.app_logs_dir.as_ref() {
        Some(path_buf) => path_buf.clone(),
        None => {
            // Handle case where app_logs_dir is not set, perhaps by setting a default or warning
            eprintln!("Application logs directory not set. Cannot log SARIF report.");
            return;
        }
    };

    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&dir_path_owned) {
        eprintln!(
            "Could not create SARIF directory '{}'. Error: {}",
            dir_path_owned.display(), // Use .display() for PathBuf formatting
            e
        );
        return;
    }

    let sarif_file_path = dir_path_owned.join("results.sarif"); // Use PathBuf::join

    let sarif_string = match serde_json::to_string_pretty(&train.sarif_report) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize SARIF report to JSON: {}", e);
            return;
        }
    };

    if let Err(e) = fs::write(&sarif_file_path, sarif_string) {
        eprintln!(
            "Failed to write SARIF report to file '{}': {}",
            sarif_file_path.display(), // Use .display() for PathBuf formatting
            e
        );
    } else {
        println!("SARIF report successfully written to '{}'", sarif_file_path.display());
    }
}
