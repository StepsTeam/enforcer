// FUNCTION: get_tool_sarif_rules
use serde_json::json;
use crate::state::Train;
use std::collections::HashMap;

static SARIF_RULES: once_cell::sync::Lazy<HashMap<&'static str, Vec<&'static str>>> = 
    once_cell::sync::Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert("cli", vec!["rule1", "rule2"]);
        m.insert("enforcer", vec!["ruleA", "ruleB"]);
        m
    });

pub fn get_tool_sarif_rules(mut train: Train) -> Train {
    let tool_name = &train.tool.tool_name;

    let rules_value = match SARIF_RULES.get(tool_name.as_str()) {
        Some(rules) => rules,
        None => {
            train.wreck.message = format!("SARIF rules not found for tool '{}'", tool_name);
            return train;
        }
    };

    train.sarif_rules = json!({ "rules": rules_value.clone() });

    train
}
