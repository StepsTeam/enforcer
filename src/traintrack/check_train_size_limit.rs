use crate::state::{Train, Warn};
use crate::debug::{watch, wreck};
use crate::tree_sitter::extract_source_code_nodes::SOURCE_CODE_NODES;

pub fn check_train_size_limit(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_train_size_limit:".to_string();
    train = watch(train);

    let nodes = match SOURCE_CODE_NODES.get() {
        Some(n) => n,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "TT_NO_SOURCE_CODE_NODES".to_string(),
                message: "Source code nodes not found. Cannot check train size limit.".to_string(),
            };
            return wreck(train);
        }
    };

    let mut found_size_limit_code = false;
    for node_info in nodes.iter() {
        let text = &node_info.text;
        if text.contains("Vec::with_capacity")
            || text.contains(".truncate")
            || text.contains(".limit")
            || text.contains("if train.len() >")
        {
            found_size_limit_code = true;
            break;
        }
    }

    if !found_size_limit_code {
        train.warn = Warn {
            level: 2,
            rule_name: "TT_NO_TRAIN_SIZE_LIMIT_CODE".to_string(),
            message: "No code found that explicitly limits the train array size (e.g., Vec::with_capacity, .truncate).".to_string(),
        };
        return train;
    }

    let max_size = train
        .config // Assumes train.config is a serde_json::Value
        .get("max_train_length")
        .and_then(|v| v.as_u64())
        .unwrap_or(1000);

    if let Some(train_array) = train.train_data.as_array() {
        if (train_array.len() as u64) > max_size {
            train.warn = Warn {
                level: 2,
                rule_name: "TT_TRAIN_TOO_BIG".to_string(),
                message: format!(
                    "Train array size ({}) exceeds the maximum allowed size of {}.",
                    train_array.len(),
                    max_size
                ),
            };
            return train;
        }
    }

    train.watch.level = 5;
    train.watch.message = "Train size is within acceptable limits".to_string();
    train = watch(train);

    train
}
