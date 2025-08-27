use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::tree_sitter::parse_source_code::PARSE_TREE;
use crate::tree_sitter::load_source_code::SOURCE_CODE;
use once_cell::sync::OnceCell;
use crate::state::Train;
use crate::state::Warn;

pub static SOURCE_CODE_NODES: OnceCell<Vec<NodeInfo>> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn extract_source_code_nodes(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "extract_source_code_nodes:".to_string();
    train = watch(train);

    let parse_tree = match PARSE_TREE.get() {
        Some(tree) => tree,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TT013".to_string(),
                message: "Parse tree not found. Cannot extract source code nodes.".to_string(),
            });
            return wreck(train);
        }
    };

    let source_code = match SOURCE_CODE.get() {
        Some(code) => code,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_SOURCE_CODE_NOT_LOADED".to_string(),
                message: "Source code is not loaded. Cannot extract node text.".to_string(),
            });
            return wreck(train);
        }
    };

    let root_node = parse_tree.root_node();
    let mut cursor = root_node.walk();
    let mut extracted_nodes: Vec<NodeInfo> = Vec::new();

    let mut stack: Vec<(tree_sitter::Node, usize)> = Vec::new();
    stack.push((root_node, 0));

    while let Some((node, depth)) = stack.pop() {
        let node_text = node.utf8_text(source_code.as_bytes())
            .unwrap_or("")
            .to_string();

        let node_info = NodeInfo {
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
        };
        extracted_nodes.push(node_info);

        let mut children_to_process: Vec<tree_sitter::Node> = node.children(&mut cursor).collect();
        children_to_process.reverse();

        for child in children_to_process {
            stack.push((child, depth + 1));
        }
    }

    if SOURCE_CODE_NODES.set(extracted_nodes).is_err() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_SOURCE_CODE_NODES_ALREADY_SET".to_string(),
            message: "Source code nodes were already set. This indicates a logic error in the pipeline.".to_string(),
        });
        return wreck(train);
    }

    train.watch.level = 5;
    train.watch.message = format!("All AST nodes extracted and stored in SOURCE_CODE_NODES ({} nodes)", SOURCE_CODE_NODES.get().unwrap().len());
    train = watch(train);

    train
}
