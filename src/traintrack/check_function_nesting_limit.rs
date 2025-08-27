use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::tree_sitter::extract_source_code_nodes::SOURCE_CODE_NODES;
use crate::state::Train;
use crate::state::Warn;

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
            train.warn_message = Some(Warn {
                rule_name: "TT006".to_string(),
                message: "Source code nodes not loaded. Cannot check function nesting limit.".to_string(),
            });
            return wreck(train);
        }
    };

    let mut function_nesting_violations: Vec<Value> = Vec::new();
    let max_nesting_limit = train.tool.nesting_limit;

    for node_info in source_code_nodes.iter() {
        if node_info.kind != "function_definition" && node_info.kind != "function_item" {
            continue;
        }

        let Some(depth) = node_info.depth else {
            continue;
        };

        if depth <= max_nesting_limit {
            continue;
        }

        let mut violation_entry = serde_json::json!({});
        violation_entry["rule_name"] = serde_json::json!("TT0021");
        violation_entry["level"] = serde_json::json!("warning");

        violation_entry["message"] = serde_json::json!(format!(
            "Function '{}' exceeds nesting limit of {}. Current depth: {}",
            node_info.text.lines().next().unwrap_or(""),
            max_nesting_limit,
            depth
        ));

        let file_path_value = train.file_path
            .as_ref()
            .map(|p| Value::String(p.to_string_lossy().into_owned()))
            .unwrap_or(serde_json::json!("unknown_file"));
        
        violation_entry["file_path"] = file_path_value;
        violation_entry["start_line"] = serde_json::json!(node_info.start_line);
        violation_entry["end_line"] = serde_json::json!(node_info.end_line);
        violation_entry["start_column"] = serde_json::json!(node_info.start_column);
        violation_entry["end_column"] = serde_json::json!(node_info.end_column);

        violation_entry["node_kind"] = serde_json::json!(node_info.kind);
        violation_entry["node_text"] = serde_json::json!(node_info.text);
        violation_entry["node_start_byte"] = serde_json::json!(node_info.start_byte);
        violation_entry["node_end_byte"] = serde_json::json!(node_info.end_byte);
        violation_entry["node_depth"] = serde_json::json!(depth);

        function_nesting_violations.push(violation_entry);
    }

    if function_nesting_violations.is_empty() {
        train.watch.level = 5;
        train.watch.message = "No function nesting violations found.".to_string();
        train = watch(train);
        return train;
    }

    if !train.warnings.is_array() {
        train.warnings = serde_json::json!([]);
    }
    if let Some(warnings_array) = train.warnings.as_array_mut() {
        for violation in &function_nesting_violations { // Fixed: Iterate by reference to avoid move
            warnings_array.push(violation.clone());
        }
    }
    train.watch.level = 5;
    train.watch.message = format!("Found {} function nesting violations.", function_nesting_violations.len());
    train = watch(train);

    train
}
