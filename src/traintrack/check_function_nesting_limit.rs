use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use crate::debug::{watch, wreck};
use crate::tree_sitter::extract_source_code_nodes::SOURCE_CODE_NODES;
use crate::state::{Train, Warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableNodeInfo {
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

pub fn check_function_nesting_limit(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_function_nesting_limit:".to_string();
    train = watch(train);

    let source_code_nodes = match SOURCE_CODE_NODES.get() {
        Some(nodes) => nodes,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "TT006".to_string(),
                message: "Source code nodes not loaded. Cannot check function nesting limit.".to_string(),
            };
            return wreck(train);
        }
    };

    let max_nesting_limit = train.tool.nesting_limit as usize;
    let mut function_nesting_violations: Vec<Value> = Vec::new();

    for node_info in source_code_nodes.iter() {
        if node_info.kind != "function_definition" && node_info.kind != "function_item" {
            continue;
        }

        let Some(depth) = node_info.depth else { continue; };
        if depth <= max_nesting_limit { continue; }

        let file_path_value = train.file_path
            .as_ref()
            .map(|p| Value::String(p.to_string_lossy().into_owned()))
            .unwrap_or(json!("unknown_file"));

        let violation_entry = json!({
            "rule_name": "TT0021",
            "level": "warning",
            "message": format!(
                "Function '{}' exceeds nesting limit of {}. Current depth: {}",
                node_info.text.lines().next().unwrap_or(""),
                max_nesting_limit,
                depth
            ),
            "file_path": file_path_value,
            "start_line": node_info.start_line,
            "end_line": node_info.end_line,
            "start_column": node_info.start_column,
            "end_column": node_info.end_column,
            "node_kind": node_info.kind,
            "node_text": node_info.text,
            "node_start_byte": node_info.start_byte,
            "node_end_byte": node_info.end_byte,
            "node_depth": depth
        });

        function_nesting_violations.push(violation_entry);
    }

    if !function_nesting_violations.is_empty() {
        if !train.warnings.is_array() {
            train.warnings = json!([]);
        }
        if let Some(arr) = train.warnings.as_array_mut() {
            arr.extend(function_nesting_violations.iter().cloned());
        }

        train.watch.level = 5;
        train.watch.message = format!("Found {} function nesting violations.", function_nesting_violations.len());
        train = watch(train);
        return train;
    }

    train.watch.level = 5;
    train.watch.message = "No function nesting violations found.".to_string();
    train = watch(train);

    train
}
