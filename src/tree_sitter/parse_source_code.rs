use crate::state::{Train, Warn};
use crate::debug::{watch, wreck};
use crate::tree_sitter::detect_language::TREE_SITTER_LANGUAGE_STORE;
use tree_sitter::{Language, Parser, Tree};
use std::fs;
use std::path::Path;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

/// Global store for the last parsed Tree-sitter tree
pub static PARSE_TREE: OnceCell<Mutex<Tree>> = OnceCell::new();

/// Parses the source code of the file in `train.file_path` using Tree-sitter.
/// Dynamically loads the grammar based on `train.tool.params["language_name"]`.
pub fn parse_source_code(mut train: Train) -> Train {
    // Early exit if train already has a wreck
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "parse_source_code: start".to_string();
    train = watch(train);

    let file_path = match &train.file_path {
        Some(p) => p,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "FILE_PATH_MISSING".to_string(),
                message: "No file path provided for parsing.".to_string(),
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

    let language_name = match train
        .tool
        .params
        .get("language_name")
        .and_then(|v| v.as_str())
    {
        Some(name) => name,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "LANGUAGE_NOT_DETECTED".to_string(),
                message: "Language name not set in train.tool.params".to_string(),
            };
            return wreck(train);
        }
    };

    // Build the path to the compiled grammar
    let lib_path = format!(
        "{}/compiled_grammars/{}_language.so",
        env!("CARGO_MANIFEST_DIR"),
        language_name
    );

    if !Path::new(&lib_path).exists() {
        train.warn = Warn {
            level: 2,
            rule_name: "LANGUAGE_LIB_NOT_FOUND".to_string(),
            message: format!("Compiled grammar not found for {} at {}", language_name, lib_path),
        };
        return wreck(train);
    }

    // Dynamically load grammar
    let language: Language = unsafe {
        let lib = match libloading::Library::new(&lib_path) {
            Ok(l) => l,
            Err(_) => {
                train.warn = Warn {
                    level: 2,
                    rule_name: "LANGUAGE_LIB_LOAD_FAILED".to_string(),
                    message: format!("Failed to load library {}", lib_path),
                };
                return wreck(train);
            }
        };
        let symbol: libloading::Symbol<unsafe extern "C" fn() -> Language> =
            match lib.get(b"tree_sitter_language") {
                Ok(s) => s,
                Err(_) => {
                    train.warn = Warn {
                        level: 2,
                        rule_name: "LANGUAGE_LIB_LOAD_FAILED".to_string(),
                        message: format!("Failed to load symbol from {}", lib_path),
                    };
                    return wreck(train);
                }
            };
        symbol()
    };

    let _ = TREE_SITTER_LANGUAGE_STORE.set(language);

    let mut parser = Parser::new();
    if parser.set_language(language).is_err() {
        train.warn = Warn {
            level: 2,
            rule_name: "PARSER_LANGUAGE_ERROR".to_string(),
            message: format!("Failed to set parser language for {}", language_name),
        };
        return wreck(train);
    }

    match parser.parse(&source_code, None) {
        Some(tree) if tree.root_node().named_child_count() > 0 => {
            // Store parsed tree globally
            let _ = PARSE_TREE.set(Mutex::new(tree.clone()));

            train.watch.level = 5;
            train.watch.message = format!("Successfully parsed source as {}", language_name);
            watch(train)
        }
        Some(_) => {
            train.warn = Warn {
                level: 2,
                rule_name: "PARSE_EMPTY_TREE".to_string(),
                message: format!("Parsing produced empty tree for {}", language_name),
            };
            wreck(train)
        }
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "PARSER_FAILED".to_string(),
                message: format!("Failed to parse file {:?}", file_path),
            };
            wreck(train)
        }
    }
}
