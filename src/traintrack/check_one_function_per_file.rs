use crate::state::{Train, Warn};
use crate::debug::{watch};

pub fn check_one_function_per_file(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_one_function_per_file:".to_string();
    train = watch(train);

    let funcs_value = match train.function_nodes.as_array() {
        Some(array) => array,
        None => {
            train.warn = Warn {
                level: 2,
                rule_name: "TT_NO_TOP_LEVEL_FUNCTION_FOUND".to_string(),
                message: "No top-level function nodes found.".to_string(),
            };
            return train;
        }
    };

    let count = funcs_value.len();

    if count > 1 {
        train.warn = Warn {
            level: 2,
            rule_name: "TT_ONE_FUNCTION_PER_FILE".to_string(),
            message: format!("More than one function ({}) found in the file.", count),
        };
        return train;
    }

    train.watch.level = 5;
    train.watch.message = format!("Number of functions: {}", count);
    train = watch(train);

    train
}
