use crate::state::{Train, Warn};
use crate::debug::{watch, wreck};
use tree_sitter::Node;

pub fn check_train_function_output(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_train_function_output: start".to_string();
    train = watch(train);

    // Access parse tree safely
    let parse_tree = match crate::tree_sitter::parse_source_code::PARSE_TREE.get() {
        Some(tree_mutex) => match tree_mutex.lock() {
            Ok(tree) => tree.clone(), // clone to avoid borrow issues
            Err(_) => {
                train.warn = Warn {
                    level: 2,
                    rule_name: "TREE_SITTER_MUTEX_LOCK_FAILED".to_string(),
                    message: "Failed to acquire parse tree lock.".to_string(),
                };
                return wreck(train);
            }
        },
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "TREE_SITTER_PARSE_TREE_NOT_FOUND".to_string(),
                message: "Parse tree not found.".to_string(),
            };
            return wreck(train);
        }
    };

    // Access source code safely
    let source_code = match crate::tree_sitter::load_source_code::SOURCE_CODE.get() {
        Some(code) => code,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "TREE_SITTER_SOURCE_CODE_NOT_LOADED".to_string(),
                message: "Source code not loaded.".to_string(),
            };
            return wreck(train);
        }
    };

    let root_node = parse_tree.root_node();
    let mut cursor = root_node.walk();

    // Example check: ensure there is only one top-level function
    let mut top_level_functions = vec![];
    for child in root_node.children(&mut cursor) {
        if child.kind() == "function_item" {
            top_level_functions.push(child);
        }
    }

    if top_level_functions.len() == 0 {
        train.warn = Warn {
            level: 2,
            rule_name: "NO_TOP_LEVEL_FUNCTION".to_string(),
            message: "No top-level function found in source code.".to_string(),
        };
        return wreck(train);
    }

    if top_level_functions.len() > 1 {
        train.warn = Warn {
            level: 2,
            rule_name: "MULTIPLE_TOP_LEVEL_FUNCTIONS".to_string(),
            message: format!("Found {} top-level functions. Only one is allowed.", top_level_functions.len()),
        };
        return wreck(train);
    }

    // Additional checks can go here (e.g., ensure parameter named 'train', ensure return 'train')
    train.watch.level = 5;
    train.watch.message = "check_train_function_output: completed".to_string();
    train = watch(train);

    train
}
