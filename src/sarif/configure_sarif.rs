use crate::state::{Train};
use crate::debug::{watch, wreck};
const SARIF_SARIF_RULES: &str = include_str!("config/sarif_rules.sarif");

pub fn configure_sarif(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "configure_sarif:".to_string();
    train = watch(train);

    // Set the SARIF tool name
    train.tool.tool_name = "sarif".to_string();

    // Optionally, parse SARIF rules from the included file and store them in train
    if let Ok(rules_json) = serde_json::from_str::<serde_json::Value>(SARIF_SARIF_RULES) {
        train.sarif_rules = rules_json;
    } else {
        train.wreck.message = "Failed to parse SARIF rules from config/sarif_rules.sarif".to_string();
        return wreck(train);
    }

    train.watch.level = 5;
    train.watch.message = "SARIF configuration completed.".to_string();
    train = watch(train);

    train
}
