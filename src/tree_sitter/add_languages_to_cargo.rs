use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json::{Value, from_str};
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::state::Train;
use crate::state::Warn;

pub fn add_languages_to_cargo(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "add_languages_to_cargo:".to_string();
    train = watch(train);

    let config_path_str = &train.tool.language_configurations_path;
    let config_path = Path::new(config_path_str);

    if config_path_str.is_empty() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_CONFIG_PATH_MISSING".to_string(),
            message: "Language configurations path is missing from train.tool.".to_string(),
        });
        return wreck(train);
    }

    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(_) => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_CONFIG_FILE_READ_FAILED".to_string(),
                message: format!("Failed to read language configurations file from '{}'.", config_path.display()),
            });
            return wreck(train);
        }
    };

    let languages_value: Value = match from_str(&config_content) {
        Ok(value) => value,
        Err(_) => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_CONFIG_FILE_INVALID_JSON".to_string(),
                message: format!("Language configurations file '{}' contains invalid JSON.", config_path.display()),
            });
            return wreck(train);
        }
    };

    let languages = match languages_value.as_object() {
        Some(langs) => langs,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_CONFIG_FILE_NOT_OBJECT".to_string(),
                message: format!("Language configurations file '{}' content is not a JSON object.", config_path.display()),
            });
            return wreck(train);
        }
    };

    let mut dep_lines = vec!["[dependencies]".to_string()];

    for (_lang_name, props) in languages.iter() {
        let Some(crate_name) = props.get("crate_name").and_then(|v| v.as_str()) else { continue; };
        let Some(crate_version) = props.get("crate_version").and_then(|v| v.as_str()) else { continue; };
        dep_lines.push(format!("{} = \"{}\"", crate_name, crate_version));
    }

    let output_path = Path::new("dependencies_snippet.toml");
    let mut file = match File::create(output_path) {
        Ok(f) => f,
        Err(_) => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_FAILED_TO_CREATE_DEPENDENCIES_SNIPPET".to_string(),
                message: format!("Failed to create dependencies snippet file at '{}'.", output_path.display()),
            });
            return wreck(train);
        }
    };

    for line in dep_lines {
        if writeln!(file, "{}", line).is_err() {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_FAILED_TO_WRITE_DEPENDENCIES_SNIPPET".to_string(),
                message: format!("Failed to write to dependencies snippet file at '{}'.", output_path.display()),
            });
            return wreck(train);
        }
    }

    train.watch.level = 5;
    train.watch.message = "Successfully added language dependencies to Cargo.toml".to_string();
    train = watch(train);

    train
}
