use tree_sitter::{Language, Parser};
use crate::state::{Train, Warn};
use crate::debug::{watch, wreck};
use once_cell::sync::OnceCell;
use serde_json::{Value, Map};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Global store for detected Language
pub static TREE_SITTER_LANGUAGE_STORE: OnceCell<Language> = OnceCell::new();

pub fn detect_language(mut train: Train) -> Train {
    // Early exit if train already has a wreck
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "detect_language: start".to_string();
    train = watch(train);

    let file_path = match &train.file_path {
        Some(p) => p,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "FILE_PATH_MISSING".to_string(),
                message: "No file path provided.".to_string(),
            };
            return wreck(train);
        }
    };

    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(_) => {
            train.warn = Warn {
                level: 2,
                rule_name: "FILE_READ_ERROR".to_string(),
                message: format!("Failed to read file {:?}", file_path),
            };
            return wreck(train);
        }
    };

    if source_code.trim().is_empty() {
        train.warn = Warn {
            level: 2,
            rule_name: "EMPTY_FILE".to_string(),
            message: format!("File {:?} is empty.", file_path),
        };
        return wreck(train);
    }

    let config_path = format!("{}/config/language_configurations.json", env!("CARGO_MANIFEST_DIR"));
    let config_json = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => {
            train.warn = Warn {
                level: 2,
                rule_name: "INVALID_LANGUAGE_CONFIG".to_string(),
                message: format!("Failed to read {}", config_path),
            };
            return wreck(train);
        }
    };

    let language_config: Map<String, Value> = match serde_json::from_str(&config_json) {
        Ok(Value::Object(map)) => map,
        _ => {
            train.warn = Warn {
                level: 2,
                rule_name: "INVALID_LANGUAGE_CONFIG".to_string(),
                message: format!("Failed to parse {}", config_path),
            };
            return wreck(train);
        }
    };

    let mut parser = Parser::new();
    if train.tool.params.is_empty() {
        train.tool.params = HashMap::new();
    }

    for (lang_name, lang_info) in language_config.iter() {
        let extensions = lang_info.get("extensions").and_then(Value::as_array).cloned().unwrap_or_default();
        let heuristics = lang_info.get("heuristics").and_then(Value::as_array).cloned().unwrap_or_default();

        let detected = extensions.iter().any(|ext| ext.as_str().map_or(false, |e| file_path.ends_with(e)))
            || heuristics.iter().any(|h| h.as_str().map_or(false, |h| source_code.contains(h)));

        if !detected {
            continue;
        }

        let lib_path = format!("{}/compiled_grammars/{}_language.so", env!("CARGO_MANIFEST_DIR"), lang_name);

        if !Path::new(&lib_path).exists() {
            train.warn = Warn {
                level: 2,
                rule_name: "LANGUAGE_LIB_NOT_FOUND".to_string(),
                message: format!("Compiled grammar not found for {} at {}", lang_name, lib_path),
            };
            continue;
        }

        let language_lib: Language = unsafe {
            match libloading::Library::new(&lib_path) {
                Ok(lib) => {
                    let func: Result<libloading::Symbol<unsafe extern "C" fn() -> Language>, _> = lib.get(b"tree_sitter_language");
                    match func {
                        Ok(symbol) => symbol(),
                        Err(_) => {
                            train.warn = Warn {
                                level: 2,
                                rule_name: "LANGUAGE_LIB_LOAD_FAILED".to_string(),
                                message: format!("Failed to load symbol from {}", lib_path),
                            };
                            continue;
                        }
                    }
                }
                Err(_) => {
                    train.warn = Warn {
                        level: 2,
                        rule_name: "LANGUAGE_LIB_LOAD_FAILED".to_string(),
                        message: format!("Failed to load library {}", lib_path),
                    };
                    continue;
                }
            }
        };

        if parser.set_language(language_lib).is_err() {
            continue;
        }

        if let Some(tree) = parser.parse(&source_code, None) {
            if tree.root_node().named_child_count() > 0 {
                train.tool.params.insert("language_name".to_string(), Value::String(lang_name.clone()));
                let _ = TREE_SITTER_LANGUAGE_STORE.set(language_lib);

                train.watch.level = 5;
                train.watch.message = format!("Detected and parsed language: {}", lang_name);
                return watch(train);
            }
        }
    }

    train.warn = Warn {
        level: 2,
        rule_name: "TREE_SITTER_NO_LANGUAGE_DETECTED".to_string(),
        message: "Could not detect or parse language using heuristics and extensions.".to_string(),
    };
    wreck(train)
}
