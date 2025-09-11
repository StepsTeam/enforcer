use crate::state::Train;
use crate::sarif::get_tool_sarif_rules;
use serde_json::Value;

/// Processes SARIF rules for the `wreck` module.
/// Ensures the tool name is set and parses the SARIF rules from `train`.
pub fn wreck(mut train: Train) -> Train {
    // Ensure the tool name is set in Train
    if train.tool.tool_name.is_empty() {
        train.tool.tool_name = "wreck".to_string();
    }

    // Populate train with SARIF rules
    train = get_tool_sarif_rules(train);

    // Attempt to parse SARIF rules JSON safely
    let rules_value: Value = match serde_json::from_str(&train.tool.traintrack_sarif_rules_str) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("cargo:warning=No SARIF rules found or failed to parse for tool: wreck");
            return train;
        }
    };

    // Safely navigate the SARIF structure
    let rules_array = rules_value
        .get("runs")
        .and_then(Value::as_array)
        .and_then(|runs| runs.get(0))
        .and_then(|first_run| first_run.get("tool"))
        .and_then(|tool| tool.get("driver"))
        .and_then(|driver| driver.get("rules"))
        .and_then(Value::as_array);

    // Process each rule if present
    if let Some(rules) = rules_array {
        for rule in rules.iter() {
            if let Some(rule_id) = rule.get("id").and_then(Value::as_str) {
                eprintln!("cargo:warning=Wreck module processing rule: {}", rule_id);
            }
        }
    }

    train
}
