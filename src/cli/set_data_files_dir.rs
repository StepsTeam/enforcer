use crate::state::{Train, Warn, Watch};
use std::path::{Path, PathBuf};
use std::fs;

pub fn set_data_files_dir(train: &mut Train, data_files_dir: &Path) {
    // Check if the provided path exists
    if !data_files_dir.exists() {
        train.warn = Warn {
            rule_name: "CLI_DATA_FILES_DIR_NOT_FOUND".to_string(),
            message: format!("Data files directory not found: {}", data_files_dir.display()),
            level: 1,
        };
        return; // Early return if path does not exist
    }

    // Check if the provided path is a directory
    if !data_files_dir.is_dir() {
        train.warn = Warn {
            rule_name: "CLI_DATA_FILES_DIR_NOT_DIRECTORY".to_string(),
            message: format!("Data files path is not a directory: {}", data_files_dir.display()),
            level: 1,
        };
        return; // Early return if path is not a directory
    }

    // If both checks pass, set the data_files_dir
    train.data_files_dir = Some(data_files_dir.to_string_lossy().into_owned());
    train.watch.message = format!("data_files_dir set to {}", train.data_files_dir.as_ref().unwrap_or(&"N/A".to_string()));
}
