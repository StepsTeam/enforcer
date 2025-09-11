use crate::state::{Train};
use std::fs::{self, File};
use std::io::Write;
use crate::debug::{watch, wreck};
use serde_json::to_string_pretty;

pub fn log_sarif_json(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "log_sarif_json:".to_string();
    train = watch(train);

    let dir_path = match &train.app_logs_dir {
        Some(path) => path,
        None => {
            train.wreck.message = "SARIF_MISSING_APP_LOGS_DIR".to_string();
            return wreck(train);
        }
    };

    if let Err(e) = fs::create_dir_all(dir_path) {
        train.wreck.message = format!(
            "SARIF_DIR_CREATE_FAILED: Could not create SARIF directory '{}'. Error: {}",
            dir_path.display(),
            e
        );
        return wreck(train);
    }

    let sarif_file_path = dir_path.join("results.sarif");

    let mut file = match File::create(&sarif_file_path) {
        Ok(f) => f,
        Err(e) => {
            train.wreck.message = format!(
                "SARIF_FILE_CREATE_FAILED: Failed to create SARIF file '{}'. Error: {}",
                sarif_file_path.display(),
                e
            );
            return wreck(train);
        }
    };

    let sarif_string = match to_string_pretty(&train.sarif_report) {
        Ok(s) => s,
        Err(e) => {
            train.wreck.message = format!(
                "SARIF_JSON_SERIALIZE_FAILED: Could not serialize SARIF JSON. Error: {}",
                e
            );
            return wreck(train);
        }
    };

    if let Err(e) = file.write_all(sarif_string.as_bytes()) {
        train.wreck.message = format!(
            "SARIF_FILE_WRITE_FAILED: Failed to write SARIF file '{}'. Error: {}",
            sarif_file_path.display(),
            e
        );
        return wreck(train);
    }

    train.watch.level = 5;
    train.watch.message = format!(
        "SARIF JSON successfully written to '{}'",
        sarif_file_path.display()
    );
    train = watch(train);

    train
}
