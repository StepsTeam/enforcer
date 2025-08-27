use crate::state::Train;
use std::path::PathBuf;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use std::env;
use crate::state::Warn;

const TRAINTRACK_SARIF_RULES: &str = include_str!("config/traintrack_rules.sarif");

pub fn configure_traintrack(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.tool.tool_name = "traintrack".to_string();

    train.watch.level = 3;
    train.watch.message = "configure_traintrack:".to_string();
    train = watch(train);

    train.tool.traintrack_sarif_rules_str = TRAINTRACK_SARIF_RULES.to_string();

    train.tool.nesting_limit = 3;

    train.watch.level = 5;
    train.watch.message = format!(
        "train[tool][tool_name] = \"{}\", nesting_limit = {}",
        &train.tool.tool_name,
        train.tool.nesting_limit
    );
    train = watch(train);

    let traintrack_config_path = PathBuf::from("src/traintrack/config/traintrack.toml");

    let current_working_dir = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown_dir".to_string()); // Fixed closure syntax

    let resolved_config_path = traintrack_config_path.canonicalize()
        .unwrap_or_else(|_| traintrack_config_path.clone());

    train.watch.level = 5;
    train.watch.message = format!(
        "Current working directory: '{}'. Attempting to load traintrack.toml from: '{}'",
        current_working_dir,
        resolved_config_path.display()
    );
    train = watch(train);

    if !traintrack_config_path.exists() {
        train.warn_message = Some(Warn {
            rule_name: "TT001".to_string(),
            message: format!(
                "Traintrack configuration file not found at '{}'. (Resolved to: '{}'). This is a fatal error.",
                traintrack_config_path.display(),
                resolved_config_path.display()
            ),
        });
        return wreck(train);
    }

    train
}
