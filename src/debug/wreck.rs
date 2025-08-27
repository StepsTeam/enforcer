use crate::state::Train;
use crate::sarif::get_tool_sarif_rules::get_tool_sarif_rules;
use serde_json::Value;

pub fn wreck(train: Train) -> Train { // Removed 'mut'
    let tool_name = "wreck";

    if let Some(rules_value) = get_tool_sarif_rules(tool_name) {
        if let Some(runs) = rules_value.get("runs") {
            if let Some(run_array) = runs.as_array() {
                if let Some(first_run) = run_array.get(0) {
                    if let Some(tool) = first_run.get("tool") {
                        if let Some(driver) = tool.get("driver") {
                            if let Some(rules_array) = driver.get("rules").and_then(Value::as_array) {
                                for rule in rules_array {
                                    if let Some(rule_id) = rule.get("id").and_then(Value::as_str) {
                                        eprintln!("cargo:warning=Wreck module processing rule: {}", rule_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("cargo:warning=No SARIF rules found for tool: {}", tool_name);

    train
}
