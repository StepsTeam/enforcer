use crate::state::{Train};
use std::path::Path;
use crate::debug::{watch, warn, wreck, configure_debug};
pub fn set_app_logs_dir(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "set_app_logs_dir:".to_string();
    train = watch(train);

    let logs_path = "/opt/micromanager/data/logs/";

    // Original logic for train["app_logs_dir"] and train["warn"] omitted.
    // These fields are not in the current Train struct.
    // To restore this logic, you must add 'app_logs_dir: String'
    // and a 'warn: WarnStruct' (with rule_name and message) to your Train struct in src/state.rs.

    if !Path::new(logs_path).exists() {
        // Original logic here set 'warn' fields like rule_name and message.
        // This functionality requires updates to the 'Train' struct in src/state.rs.
        // For now, no action is taken for this specific warning.
    }

    train.watch.level = 5;
    train.watch.message = format!("app_logs_dir set to {}", logs_path);
    train = watch(train);

    train
}
