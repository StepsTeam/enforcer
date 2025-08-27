use crate::debug::watch::watch;
use crate::debug::wreck::wreck;
use crate::state::{Train, Warn};
use crate::tree_sitter::load_source_code::SOURCE_CODE;
use once_cell::sync::OnceCell;
use tree_sitter::{Parser, Tree, Language};

pub static PARSE_TREE: OnceCell<Tree> = OnceCell::new();

// Declare external C functions for tree-sitter grammars
// ONLY FOR THE LIMITED SET: Bash, JSON, PHP, Rust, YAML
extern "C" {
    fn tree_sitter_bash() -> Language;
    fn tree_sitter_json() -> Language; // Added JSON
    fn tree_sitter_php() -> Language;  // Added PHP
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_yaml() -> Language; // Added YAML
}

pub fn parse_source_code(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "parse_source_code:".to_string();
    train = watch(train);

    // Clone the language name from the train.tool struct
    let language_name_from_train = train.tool.language_name.clone();

    // Dynamically get an OWNED Language object based on the detected name
    let owned_language_instance: Language = match language_name_from_train.as_str() {
        "bash" => unsafe { tree_sitter_bash() },
        "json" => unsafe { tree_sitter_json() }, // Use the JSON language
        "php" => unsafe { tree_sitter_php() },   // Use the PHP language
        "rust" => unsafe { tree_sitter_rust() },
        "yaml" => unsafe { tree_sitter_yaml() }, // Use the YAML language
        _ => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_UNKNOWN_LANGUAGE".to_string(),
                message: format!("Unknown or unsupported language: '{}'. Cannot parse source code.", language_name_from_train),
            });
            return wreck(train);
        }
    };

    // Use a reference to the owned Language instance for .name()
    let current_language_name = (&owned_language_instance).name().to_string();

    let source_code = match SOURCE_CODE.get() {
        Some(code) => code,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_SOURCE_CODE_NOT_LOADED".to_string(),
                message: "Source code is not loaded. Ensure load_source_code was successful.".to_string(),
            });
            return wreck(train);
        }
    };

    if source_code.trim().is_empty() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_SOURCE_CODE_EMPTY".to_string(),
            message: "Source code is empty or only whitespace. Cannot parse.".to_string(),
        });
        return wreck(train);
    }

    let mut parser = Parser::new();

    // Move the OWNED Language instance into set_language.
    // 'owned_language_instance' is consumed here.
    if parser.set_language(owned_language_instance).is_err() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_FAILED_TO_SET_LANGUAGE".to_string(),
            message: format!("Failed to set Tree-sitter language '{}' on parser.", current_language_name),
        });
        return wreck(train);
    }

    let tree = match parser.parse(source_code, None) {
        Some(t) => t,
        None => {
            train.warn_message = Some(Warn {
                rule_name: "TREE_SITTER_FAILED_TO_PARSE_SOURCE_CODE".to_string(),
                message: format!("Failed to parse source code for language '{}'.", current_language_name),
            });
            return wreck(train);
        }
    };

    if PARSE_TREE.set(tree.clone()).is_err() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_PARSE_TREE_ALREADY_SET".to_string(),
            message: "Parse tree was already set. This indicates a logic error in the pipeline.".to_string(),
        });
        return wreck(train);
    }

    eprintln!("cargo:warning=Symbolic Expression: {}", tree.root_node().to_sexp());

    train.watch.level = 5;
    train.watch.message = format!(
        "Parsed source code for language '{}', root node '{}', {} child nodes",
        current_language_name,
        tree.root_node().kind(),
        tree.root_node().child_count()
    );

    train = watch(train);
    train
}
