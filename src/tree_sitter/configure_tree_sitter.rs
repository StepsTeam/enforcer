use crate::state::{Train};
use crate::debug::{watch};
use serde_json::Value;

pub fn configure_tree_sitter(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 2;
    train.watch.message = "configure_tree_sitter: start".to_string();
    train = watch(train);

    let config_path = train
        .tool
        .params
        .get("language_configurations_path")
        .and_then(|v| v.as_str())
        .unwrap_or("config/language_configurations.json");

    let config_str = match std::fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(_) => {
            train.warnings = Value::Array(vec![serde_json::json!({
                "rule_name": "CONFIG_FILE_MISSING",
                "artifact_url": config_path,
                "message": format!("Could not read Tree-sitter configuration file: {}", config_path)
            })]);
            train.watch.message = "Failed to read Tree-sitter config, returning early".to_string();
            return train;
        }
    };

    let config_json: Value = match serde_json::from_str(&config_str) {
        Ok(v) => v,
        Err(e) => {
            train.warnings = Value::Array(vec![serde_json::json!({
                "rule_name": "CONFIG_JSON_INVALID",
                "artifact_url": config_path,
                "message": format!("Failed to parse Tree-sitter configuration JSON: {}", e)
            })]);
            train.watch.message = "Invalid Tree-sitter JSON, returning early".to_string();
            return train;
        }
    };

    train.tool.params.insert("tree_sitter_config".to_string(), config_json);

    train.watch.level = 4;
    train.watch.message = "Tree-sitter configuration applied successfully".to_string();
    train = watch(train);

    train
}
