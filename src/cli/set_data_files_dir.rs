use crate::state::{Train, Warn};
use std::path::PathBuf;

pub fn set_data_files_dir(mut train: Train) -> Train {
    // Example: derive the directory path (could be from config, CLI args, or default)
    let data_files_dir = PathBuf::from("data");

    // Check if the provided path exists
    if !data_files_dir.exists() {
        train.warn = Warn {
            rule_name: "CLI_DATA_FILES_DIR_NOT_FOUND".to_string(),
            message: format!(
                "Data files directory not found: {}",
                data_files_dir.display()
            ),
            level: 1,
        };
        return train;
    }

    // Check if the provided path is a directory
    if !data_files_dir.is_dir() {
        train.warn = Warn {
            rule_name: "CLI_DATA_FILES_DIR_NOT_DIRECTORY".to_string(),
            message: format!(
                "Data files path is not a directory: {}",
                data_files_dir.display()
            ),
            level: 1,
        };
        return train;
    }

    // If both checks pass, set the data_files_dir
    train.data_files_dir = Some(data_files_dir.to_string_lossy().into_owned());
    train.watch.message = format!(
        "data_files_dir set to {}",
        train.data_files_dir
            .as_ref()
            .unwrap_or(&"N/A".to_string())
    );

    train
}
