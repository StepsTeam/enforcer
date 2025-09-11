use crate::state::{Train, Warn};
use crate::debug::{watch, wreck};
use once_cell::sync::OnceCell;
use tree_sitter::Node;

// Global store for extracted AST nodes
pub static SOURCE_CODE_NODES: OnceCell<Vec<NodeInfo>> = OnceCell::new();

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfo {
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<usize>,
}

pub fn extract_source_code_nodes(train: Train) -> Train {
    // Early exit if train has a wreck
    if !train.wreck.message.is_empty() {
        return train;
    }

    let mut train = train;
    train.watch.level = 3;
    train.watch.message = "extract_source_code_nodes: start".to_string();
    train = watch(train);

    // Access parse tree safely
    let parse_tree = match crate::tree_sitter::parse_source_code::PARSE_TREE.get() {
        Some(tree_mutex) => match tree_mutex.lock() {
            Ok(tree) => tree.clone(), // clone the tree to avoid borrow issues
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
    let mut extracted_nodes: Vec<NodeInfo> = Vec::new();
    let mut stack: Vec<(Node, usize)> = vec![(root_node, 0)];

    while let Some((node, depth)) = stack.pop() {
        let node_text = node.utf8_text(source_code.as_bytes()).unwrap_or("").to_string();

        extracted_nodes.push(NodeInfo {
            kind: node.kind().to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_line: node.start_position().row,
            end_line: node.end_position().row,
            start_column: node.start_position().column,
            end_column: node.end_position().column,
            text: node_text,
            parent_kind: node.parent().map(|p| p.kind().to_string()),
            children_count: Some(node.child_count()),
            depth: Some(depth),
        });

        let mut children: Vec<Node> = node.children(&mut cursor).collect();
        children.reverse();
        for child in children {
            stack.push((child, depth + 1));
        }
    }

    if SOURCE_CODE_NODES.set(extracted_nodes).is_err() {
        train.warn = Warn {
            level: 2,
            rule_name: "TREE_SITTER_SOURCE_CODE_NODES_ALREADY_SET".to_string(),
            message: "Source code nodes already set. Pipeline logic error.".to_string(),
        };
        return wreck(train);
    }

    train.watch.level = 5;
    train.watch.message = format!(
        "All AST nodes extracted and stored ({} nodes)",
        SOURCE_CODE_NODES.get().unwrap().len()
    );
    watch(train)
}
