use crate::state::{Train, Warn};

pub fn check_train_function_input(mut train: Train) -> Train {
    let Some(value) = train.tool.params.get("language_name") else {
        train.warn = Warn {
            level: 2,
            rule_name: "LANGUAGE_PARAM_MISSING".to_string(),
            message: "The 'language_name' parameter is missing from train.tool.params".to_string(),
        };
        return train;
    };

    let Some(language_name) = value.as_str() else {
        train.warn = Warn {
            level: 2,
            rule_name: "LANGUAGE_PARAM_INVALID".to_string(),
            message: "The 'language_name' parameter must be a string".to_string(),
        };
        return train;
    };

    if language_name.is_empty() {
        train.warn = Warn {
            level: 2,
            rule_name: "LANGUAGE_PARAM_EMPTY".to_string(),
            message: "The 'language_name' parameter is empty".to_string(),
        };
        return train;
    }

    train
}
