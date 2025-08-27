use crate::state::Train;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use serde_json::Value;

pub fn log_sarif_json(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "log_sarif_json:".to_string();
    train = watch(train);

    let dir_path_owned = match train.app_logs_dir.as_ref() {
        Some(path) => path.clone(),
        None => {
            train.wreck.message = "SARIF_MISSING_APP_LOGS_DIR".to_string();
            return wreck(train);
        }
    };

    if let Err(e) = fs::create_dir_all(&dir_path_owned) {
        train.wreck.message = format!("SARIF_DIR_CREATE_FAILED: Could not create SARIF directory '{}'. Error: {}", dir_path_owned, e);
        return wreck(train);
    }

    let sarif_file_path = format!("{}/results.sarif", dir_path_owned);

    let mut file = match File::create(&sarif_file_path) {
        Ok(f) => f,
        Err(e) => {
            train.wreck.message = format!("SARIF_FILE_CREATE_FAILED: Failed to create SARIF file '{}'. Error: {}", sarif_file_path, e);
            return wreck(train);
        }
    };

    let sarif_string = match serde_json::to_string_pretty(&train.sarif_report) {
        Ok(s) => s,
        Err(e) => {
            train.wreck.message = format!("SARIF_JSON_SERIALIZE_FAILED: Could not serialize SARIF JSON. Error: {}", e);
            return wreck(train);
        }
    };

    if let Err(e) = file.write_all(sarif_string.as_bytes()) {
        train.wreck.message = format!("SARIF_FILE_WRITE_FAILED: Failed to write SARIF file '{}'. Error: {}", sarif_file_path, e);
        return wreck(train);
    }

    train
}
