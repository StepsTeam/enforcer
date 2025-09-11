use crate::state::{Train};
use crate::debug::{watch};
use serde_json::Value;

/// Validates and resets train.warnings into proper SARIF-like format
/// Ensures each warning has required fields and a valid artifact URL
pub fn reset_train_warnings(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "reset_train_warnings:".to_string();
    train = watch(train);

    if !train.warnings.is_array() {
        train.warnings = Value::Null;
        return train;
    }

    let warnings_array = train.warnings.as_array().cloned().unwrap_or_default();
    if warnings_array.is_empty() {
        train.warnings = Value::Null;
        return train;
    }

    let file_path_abs = train.file_path
        .as_ref()
        .map(|pb| pb.to_string_lossy().into_owned())
        .unwrap_or_default();

    let mut artifact_url = String::new();
    if !file_path_abs.is_empty() {
        artifact_url = if cfg!(windows) {
            format!("file:///{}", file_path_abs.replace("\\", "/"))
        } else {
            format!("file://{}", file_path_abs)
        };
    }

    let required_fields = [
        "tool_name", "tool_version", "tool_url",
        "rule_id", "rule_name", "short_description", "full_description", "severity_level",
        "artifact_url", "start_line", "end_line", "start_column", "end_column",
        "help_url", "message", "prompt",
    ];

    let mut valid_warnings = Vec::new();

    for warning in warnings_array {
        if !warning.is_object() {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_INVALID_WARNING_OBJECT",
                "artifact_url": artifact_url
            }));
            continue;
        }

        if file_path_abs.is_empty() {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_MISSING_FILE_PATH",
                "artifact_url": artifact_url
            }));
            continue;
        }

        let warning_obj = warning.as_object().unwrap();
        let missing_field = required_fields.iter().any(|f| !warning_obj.contains_key(*f));

        if missing_field {
            valid_warnings.push(serde_json::json!({
                "rule_name": "SARIF_MISSING_FIELD",
                "artifact_url": artifact_url
            }));
            continue;
        }

        valid_warnings.push(warning);
    }

    train.warnings = if valid_warnings.is_empty() {
        Value::Null
    } else {
        Value::Array(valid_warnings)
    };

    train
}
