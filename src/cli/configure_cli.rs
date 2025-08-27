use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;

const CLI_SARIF_RULES: &str = include_str!("config/cli_rules.sarif");

pub fn configure_cli(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return wreck(train);
    }

    train.watch.level = 3;
    train.watch.message = "configure_cli:".to_string();
    train = watch(train);

    train.tool.tool_name = "cli".to_string();

    train.watch.level = 5;
    train.watch.message = format!("train[tool][tool_name] = {}", &train.tool.tool_name);
    train = watch(train);

    train
}
