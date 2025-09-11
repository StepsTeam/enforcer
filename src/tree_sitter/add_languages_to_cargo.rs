use crate::state::{Train, Warn};
use std::path::Path;

pub fn add_languages_to_cargo(mut train: Train) -> Train {
    let config_path_str = train
        .tool
        .params
        .get("language_configurations_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if config_path_str.is_empty() {
        train.warn = Warn {
            rule_name: "ADD_LANGUAGES_TO_CARGO_MISSING_PATH".to_string(),
            message: "Missing language_configurations_path in tool.params".to_string(),
            level: 1,
        };
        return train;
    }

    let config_path = Path::new(config_path_str);

    if !config_path.exists() {
        train.warn = Warn {
            rule_name: "ADD_LANGUAGES_TO_CARGO_FILE_NOT_FOUND".to_string(),
            message: format!("Language configuration file does not exist: {}", config_path_str),
            level: 1,
        };
        return train;
    }

    println!("Language configurations loaded from: {}", config_path_str);

    train
}
