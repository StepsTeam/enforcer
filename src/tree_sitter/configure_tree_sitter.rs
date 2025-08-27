use crate::state::Train;
use crate::debug::watch::watch; // Import the watch function
use crate::debug::wreck::wreck; // Import the wreck function

/// Configures Tree-sitter related settings within the pipeline.
pub fn configure_tree_sitter(mut train: Train) -> Train { // Corrected from Value to Train
    // If a "wreck" condition exists, propagate it immediately.
    if !train.wreck.message.is_empty() {
        return wreck(train); // Call wreck to process/log the wreck and return train
    }

    // Log the entry into the configure_tree_sitter function.
    train.watch.level = 3;
    train.watch.message = "configure_tree_sitter:".to_string();
    train = watch(train);

    // Ensure `train.tool` is properly configured.
    // The `Train::new()` already initializes `train.tool` as a `Tool` struct,
    // so explicit JSON object creation is no longer needed here.
    // We can directly assign values to its fields.

    train.tool.tool_name = "tree_sitter".to_string();

    train.tool.language_configurations_path = "config/language_configurations.json".to_string();

    // Log the configured language configurations path.
    train.watch.level = 5;
    train.watch.message = format!(
        "language_configurations_path = {}",
        &train.tool.language_configurations_path
    );
    train = watch(train);

    train
}
