use serde_json::{Value, json};
use crate::SARIF_RULES;
use crate::state::Train;

pub fn get_tool_sarif_rules(tool_name: &str) -> Option<Value> {
    let tool_rules = SARIF_RULES.get(tool_name);
    
    match tool_rules {
        Some(rules_value) => Some(json!({ "rules": [rules_value] })),
        None => None,
    }
}