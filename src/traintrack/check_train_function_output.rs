use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::state::Warn;
use crate::tree_sitter::extract_source_code_nodes::SOURCE_CODE_NODES;
use crate::tree_sitter::parse_source_code::PARSE_TREE;
use crate::tree_sitter::load_source_code::SOURCE_CODE;
use tree_sitter::Node;
use serde_json::Value;

pub fn check_train_function_output(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_train_function_output:".to_string();
    train = watch(train);

    let tree = match PARSE_TREE.get() {
        Some(t) => t,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TT_TREE_SITTER_PARSE_TREE_NOT_FOUND".to_string(),
                message: "Parse tree not found. Cannot check function output.".to_string(),
            });
            return wreck(train);
        }
    };

    let nodes = match SOURCE_CODE_NODES.get() {
        Some(n) => n,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TT_NO_SOURCE_CODE_NODES".to_string(),
                message: "Source code nodes not loaded. Cannot check function output.".to_string(),
            });
            return wreck(train);
        }
    };

    let forbidden_else_kinds = [
        "else_clause", "else_statement", "elif_clause", "elseif_clause", "el_if",
    ];
    for node_info in nodes.iter() {
        if forbidden_else_kinds.iter().any(|&k| k == node_info.kind) {
            train.warn_message = Some(Warn {
                rule_name: "TT_AVOID_ELSEIF_ELSE".to_string(),
                message: format!("Forbidden 'else-like' keyword found: {}", node_info.kind),
            });
            return train;
        }
    }

    let forbidden_switch_kinds = [
        "switch_statement", "case_clause", "match_expression",
    ];
    for node_info in nodes.iter() {
        if forbidden_switch_kinds.iter().any(|&k| k == node_info.kind) {
            train.warn_message = Some(Warn {
                rule_name: "TT_AVOID_CASE_SWITCHES".to_string(),
                message: format!("Forbidden 'switch/case-like' keyword found: {}", node_info.kind),
            });
            return train;
        }
    }

    let root_node = tree.root_node();

    let mut returns_nodes = Vec::new();
    let mut stack = vec![root_node];
    while let Some(node) = stack.pop() {
        if node.kind() == "return_statement" || node.kind() == "return_expression" {
            returns_nodes.push(node);
            continue;
        }
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                stack.push(child);
            }
        }
    }

    if returns_nodes.is_empty() {
        train.warn_message = Some(Warn {
            rule_name: "TT_RETURN_TRAIN_ARRAY".to_string(),
            message: "No return statements found. All functions must return train.".to_string(),
        });
        return train;
    }

    let source_code = SOURCE_CODE.get().map(|s| s.as_str()).unwrap_or("");

    for ret_node in returns_nodes.iter() {
        fn collect_identifiers(node: Node, source_code: &str, identifiers: &mut Vec<String>) {
            if node.kind() == "identifier" {
                if let Ok(text) = node.utf8_text(source_code.as_bytes()) {
                    identifiers.push(text.to_string());
                }
                return;
            }
            if node.kind() == "tuple_expression" || node.kind() == "parenthesized_expression" || node.kind() == "expression_list" {
                for i in 0..node.child_count() {
                    if let Some(child) = node.child(i) {
                        collect_identifiers(child, source_code, identifiers);
                    }
                }
                return;
            }
        }

        if ret_node.child_count() == 0 {
            train.warn_message = Some(Warn {
                rule_name: "TT_RETURN_TRAIN_ARRAY".to_string(),
                message: "Empty return statement found. All functions must return train.".to_string(),
            });
            return train;
        }

        let expr_node = ret_node.child(0).unwrap();
        let mut idents = Vec::new();
        collect_identifiers(expr_node, source_code, &mut idents);

        if idents.len() != 1 || idents[0] != "train" {
            train.warn_message = Some(Warn {
                rule_name: "TT_MULTIPLE_OUTPUTS".to_string(),
                message: format!("Function returns multiple outputs or an unexpected value: {:?}", idents),
            });
            return train;
        }
    }

    train.watch.level = 5;
    train.watch.message = "Function output correctly returns only the train array".to_string();
    train = watch(train);

    train
}
