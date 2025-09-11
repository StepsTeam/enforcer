// /opt/enforcer/src/tree_sitter/mod.rs

use once_cell::sync::Lazy;
use tree_sitter::Tree;
use std::sync::Mutex;

pub mod add_languages_to_cargo;
pub mod configure_tree_sitter;
pub mod track_tree_sitter;

// Only include modules that exist
pub mod detect_language;
pub mod load_source_code;
pub mod parse_source_code;
pub mod extract_source_code_nodes;

// Re-export the function if needed, otherwise remove to avoid unused import warning
// Commenting out to fix unused import warning
// pub use track_tree_sitter::track_tree_sitter;

// Global state
pub static PARSE_TREE: Lazy<Mutex<Option<Tree>>> = Lazy::new(|| Mutex::new(None));
pub static SOURCE_CODE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
