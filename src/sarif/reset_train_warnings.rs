use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use serde_json::Value;

pub fn reset_train_warnings(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "reset_train_warnings:".to_string();
    train = watch(train);

    if !train.warnings.is_array() {
        return train;
    }

    if train.warnings.as_array().map_or(true, |arr| arr.is_empty()) {
        train.warnings = Value::Null;
        return train;
    }

    let warnings_array_ref = train.warnings.as_array().unwrap().clone();
    let mut valid_warnings = Vec::new();

    let file_path_abs = train.file_path
        .as_ref()
        .map(|pb| pb.to_string_lossy().into_owned())
        .unwrap_or_else(String::new);

    let mut artifact_url = String::new();
    if !file_path_abs.is_empty() {
        artifact_url = format!("file://{}", file_path_abs);
        if cfg!(windows) {
            artifact_url = format!("file:///{}", file_path_abs.replace("\\", "/"));
        }
    }

    let required_fields = [
        "tool_name", "tool_version", "tool_url",
        "rule_id", "rule_name", "short_description", "full_description", "severity_level",
        "artifact_url", "start_line", "end_line", "start_column", "end_column",
        "help_url", "message", "prompt",
    ];

    for error in warnings_array_ref {
        if !error.is_object() {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_INVALID_ERROR_OBJECT",
                "artifact_url": &artifact_url
            }));
            continue;
        }

        if file_path_abs.is_empty() {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_MISSING_FILE_PATH",
                "artifact_url": &artifact_url
            }));
            continue;
        }

        let mut missing_field_found = false;
        if let Some(error_obj) = error.as_object() {
            for field in &required_fields {
                if !error_obj.contains_key(*field) {
                    missing_field_found = true;
                    break;
                }
            }
        }

        if missing_field_found {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_MISSING_FIELD",
                "artifact_url": &artifact_url
            }));
            continue;
        }

        valid_warnings.push(error);
    }

    if valid_warnings.is_empty() {
        train.warnings = Value::Null; 
        return train;
    }
    
    train.warnings = Value::Array(valid_warnings);
    
    train
}
