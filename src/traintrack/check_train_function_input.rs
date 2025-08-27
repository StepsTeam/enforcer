use crate::state::Train;
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::tree_sitter::extract_source_code_nodes::SOURCE_CODE_NODES;
use crate::state::Warn; // This struct needs to be defined in src/state.rs

pub fn check_train_function_input(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "check_train_function_input:".to_string();
    train = watch(train);

    let params_option = train.tool.params.as_deref(); // Assumes train.tool.params is Option<String>
    let params = params_option.unwrap_or("");

    if !(params.contains("Vec") || (params.contains("[") && params.contains("]"))) {
        train.warn_message = Some(Warn {
            rule_name: "TT_FUNCTION_PARAM_MUST_BE_VEC_OR_ARRAY".to_string(),
            message: format!("Function parameters '{}' must contain 'Vec' or array-like syntax '[...]'.", params),
        });
        return train;
    }

    let nodes = match SOURCE_CODE_NODES.get() {
        Some(n) => n,
        None => {
            train.wreck.message = "TT_NO_SOURCE_CODE_NODES: Source code nodes not available. Cannot check function input.".to_string();
            return wreck(train);
        }
    };

    if nodes.iter().find(|n| n.kind == "function_item" || n.kind == "function_declaration").is_none() {
        train.wreck.message = "TT_FUNCTION_NODE_NOT_FOUND: No function node found in AST. Cannot check function input.".to_string();
        return wreck(train);
    }

    train.watch.level = 5;
    train.watch.message = "Function input parameters validated".to_string();
    train = watch(train);

    train
}
