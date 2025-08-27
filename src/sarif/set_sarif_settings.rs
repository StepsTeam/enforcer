use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use serde_json::Value;

include!(concat!(env!("OUT_DIR"), "/static_sarif_rules.rs"));

pub fn set_sarif_settings(mut train: Train) -> Train {
    train.watch.message = "set_sarif_settings: start".to_string();
    train = watch(train);

    if !train.wreck.message.is_empty() {
        train.watch.message = "train contains wreck, returning early".to_string();
        train = watch(train);
        return train;
    }

    let original_tool_name = train.tool.tool_name.clone();
    
    if original_tool_name.is_empty() {
        train.watch.message = "tool_name not set in train".to_string();
        train = watch(train);
        train.wreck.message = "SARIF_TOOL_NAME_NOT_SET: tool_name not set in train".to_string();
        return wreck(train);
    }
    train.watch.message = format!("Found tool_name: {}", original_tool_name).to_string();
    train = watch(train);

    let tool_key = original_tool_name.to_lowercase()
        .replace(' ', "_")
        .replace('-', "_");
    train.watch.message = format!("Normalized tool_key for lookup: {}", tool_key).to_string();
    train = watch(train);

    train.watch.message = "Listing embedded SARIF_RULES keys:".to_string();
    train = watch(train);

    for (key, _) in crate::SARIF_RULES.entries() {
        train.watch.message = format!(" - {}", key).to_string();
        train = watch(train);
    }

    train.watch.message = format!("Looking up SARIF rules for key: {}", tool_key).to_string();
    train = watch(train);

    let sarif_rules_str = match crate::SARIF_RULES.get(tool_key.as_str()) {
        Some(s) => {
            train.watch.message = format!("Found SARIF rules entry for key: {}", tool_key).to_string();
            train = watch(train);
            *s
        },
        None => {
            train.watch.message = format!("SARIF rules missing for key: {}", tool_key).to_string();
            train = watch(train);
            train.wreck.message = format!("SARIF_RULES_NOT_CONFIGURED_FOR_TOOL: SARIF rules missing for key: {}", tool_key).to_string();
            return wreck(train);
        }
    };

    let sarif_object: Value = match serde_json::from_str(sarif_rules_str) {
        Ok(v) => {
            train.watch.message = format!("SARIF JSON parsed successfully for tool '{}'", original_tool_name).to_string();
            train = watch(train);
            v
        },
        Err(e) => {
            train.watch.message = format!("SARIF JSON parsing failed for '{}': {}", original_tool_name, e).to_string();
            train = watch(train);
            train.wreck.message = format!(
                "SARIF_RULES_INVALID_JSON: SARIF rules JSON parsing failed for tool '{}': {}",
                original_tool_name, e
            ).to_string();
            return wreck(train);
        }
    };

    let driver = sarif_object
        .get("runs")
        .and_then(|runs| runs.get(0))
        .and_then(|run| run.get("tool"))
        .and_then(|tool| tool.get("driver"));
    train.watch.message = format!("Extracted driver info: {:?}", driver).to_string();
    train = watch(train);

    let name = driver
        .and_then(|d| d.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or(original_tool_name.as_str());
    train.watch.message = format!("Driver name: {}", name).to_string();
    train = watch(train);

    let version = driver
        .and_then(|d| d.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");
    train.watch.message = format!("Driver version: {}", version).to_string();
    train = watch(train);

    let url = driver
        .and_then(|d| d.get("informationUri"))
        .and_then(|u| u.as_str())
        .unwrap_or("");
    train.watch.message = format!("Driver informationUri: {}", url).to_string();
    train = watch(train);

    train.tool.tool_name = name.to_string();
    // These next two lines will cause errors. You need to add tool_version and tool_url to the Tool struct in src/state.rs
    train.tool.tool_version = version.to_string(); 
    train.tool.tool_url = url.to_string();
    train.watch.message = "Updated train with SARIF tool info".to_string();
    train = watch(train);

    train.watch.message = "set_sarif_settings: completed".to_string();
    train = watch(train);

    train
}
