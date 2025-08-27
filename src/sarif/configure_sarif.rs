use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck; // Assuming wreck is used implicitly if train.wreck is set

const SARIF_SARIF_RULES: &str = include_str!("config/sarif_rules.sarif");

pub fn configure_sarif(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "configure_sarif:".to_string();
    train = watch(train);

    train.tool.tool_name = "sarif".to_string();

    train
}
