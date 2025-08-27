use tree_sitter::{Language, Parser};
use crate::state::Train;
use crate::debug::watch::watch; // Assuming watch is still used
use crate::debug::wreck::wreck; // Assuming wreck is still used
use crate::state::Warn; // Assuming Warn is used for messages
use once_cell::sync::OnceCell; // For global storage of detected Language

// Global to hold the tree-sitter Language object once it's detected
pub static TREE_SITTER_LANGUAGE_STORE: OnceCell<Language> = OnceCell::new();

// Declare external C functions for tree-sitter grammars
// ONLY FOR THE LIMITED SET: Bash, JSON, PHP, Rust, YAML
extern "C" {
    fn tree_sitter_bash() -> Language;
    fn tree_sitter_json() -> Language;
    fn tree_sitter_php() -> Language;
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_yaml() -> Language;
}

pub fn detect_language(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "detect_language:".to_string();
    train = watch(train);

    let Some(file_path) = train.file_path.as_ref() else {
        train.warn_message = Some(Warn {
            rule_name: "FILE_PATH_MISSING".to_string(),
            message: "No file path provided to detect language.".to_string(),
        });
        return wreck(train);
    };

    let source_code = std::fs::read_to_string(file_path).unwrap_or_default();
    if source_code.is_empty() {
        train.warn_message = Some(Warn {
            rule_name: "EMPTY_FILE".to_string(),
            message: format!("File {:?} is empty. Cannot detect language.", file_path),
        });
        return wreck(train);
    }


    let mut parser = Parser::new();
    let mut detected_language_name: Option<String> = None;
    let mut detected_language_obj: Option<Language> = None;

    // Limited set of languages based on language_configurations-json-limited-set
    let languages_to_try: Vec<(&str, unsafe fn() -> Language)> = vec![
        ("bash", tree_sitter_bash),
        ("json", tree_sitter_json),
        ("php", tree_sitter_php),
        ("rust", tree_sitter_rust),
        ("yaml", tree_sitter_yaml),
    ];

    for (name, language_fn) in languages_to_try {
        let language_lib = unsafe { language_fn() }; // This gets the Language object
        
        // Attempt to parse the source code with this language
        if let Err(e) = parser.set_language(language_lib) {
            eprintln!("cargo:warning=Failed to set language {}: {}", name, e);
            continue;
        }

        if let Some(tree) = parser.parse(&source_code, None) {
            // Check if the parse tree has any named children, indicating a successful parse
            if tree.root_node().named_child_count() > 0 {
                detected_language_name = Some(name.to_string());
                detected_language_obj = Some(language_lib); // Store the Language object
                break; // Found a working language, stop trying others
            }
        }
    }

    // If no language was successfully detected after trying all
    if detected_language_name.is_none() {
        train.warn_message = Some(Warn {
            rule_name: "TREE_SITTER_NO_LANGUAGE_DETECTED".to_string(),
            message: "Could not detect language for the given file using the configured grammars.".to_string(),
        });
        return wreck(train);
    }

    // If a language was detected, store its name in train.tool and the Language object in the global OnceCell
    if let Some(lang_name) = detected_language_name {
        train.tool.language_name = lang_name; // Store the name in the train state
        if let Some(lang_obj) = detected_language_obj {
            // Attempt to set the global OnceCell with the detected Language object
            if TREE_SITTER_LANGUAGE_STORE.set(lang_obj).is_err() {
                train.warn_message = Some(Warn {
                    rule_name: "TREE_SITTER_LANGUAGE_ALREADY_SET".to_string(),
                    message: "Tree-sitter Language was already set in global store. This indicates a logic error.".to_string(),
                });
                return wreck(train);
            }
        }
        train.watch.message = format!("Detected language: {}", train.tool.language_name);
        train = watch(train);
    }

    train
}
