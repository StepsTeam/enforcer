use once_cell::sync::OnceCell;
use std::fs;
use crate::state::{Train}; // Corrected: Use Train struct instead of serde_json::Value
use crate::debug::{watch, wreck};

/// Global buffer for the loaded source code content.
pub static SOURCE_CODE: OnceCell<String> = OnceCell::new();

/// Global buffer for the loaded source file path.
pub static SOURCE_FILE_PATH: OnceCell<String> = OnceCell::new();

/// Loads the source code from the specified file path into global buffers.
pub fn load_source_code(mut train: Train) -> Train { // Corrected: Accepts and returns Train
    // If a "wreck" condition exists (message is not empty), propagate it immediately.
    if !train.wreck.message.is_empty() {
        return wreck(train);
    }

    train.watch.level = 3; // Direct assignment to u8 field
    train.watch.message = "load_source_code:".to_string(); // Direct assignment to String field
    train = watch(train);

    // Get file_path from train.file_path (which is Option<PathBuf>)
    let file_path_buf = match train.file_path.clone() { // Clone the PathBuf to own it
        Some(path_buf) => path_buf,
        None => {
            // Original: train["warn"]["rule_name"] = json!("TREE_SITTER_NO_FILE_PATH");
            // Translate to setting wreck message and returning wreck(train)
            train.wreck.message = "No file path for Tree-sitter. Rule: TREE_SITTER_NO_FILE_PATH".to_string();
            return wreck(train);
        }
    };

    // Read file content from disk
    match fs::read_to_string(&file_path_buf) { // Use &PathBuf for fs::read_to_string
        Ok(content) => {
            // Ignore if already set
            let _ = SOURCE_CODE.set(content);
            let _ = SOURCE_FILE_PATH.set(file_path_buf.to_string_lossy().into_owned()); // Convert PathBuf to String for OnceCell
        }
        Err(e) => {
            // Original: train["warn"]["rule_name"] = json!("TREE_SITTER_FAILED_TO_READ_SOURCE");
            // Translate to setting wreck message and returning wreck(train)
            train.wreck.message = format!("Failed to read source from {:?}. Rule: TREE_SITTER_FAILED_TO_READ_SOURCE. Error: {}", file_path_buf, e);
            return wreck(train);
        }
    }

    train
}
