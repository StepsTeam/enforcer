use once_cell::sync::Lazy;
use tree_sitter::Tree;
use std::sync::Mutex;

pub mod detect_language;
pub mod extract_source_code_nodes;
pub mod load_source_code;
pub mod parse_source_code;
pub mod add_languages_to_cargo;
pub mod configure_tree_sitter;
pub mod track_tree_sitter;

pub static PARSE_TREE: Lazy<Mutex<Option<Tree>>> = Lazy::new(|| Mutex::new(None));
pub static SOURCE_CODE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub use detect_language::detect_language;
pub use extract_source_code_nodes::extract_source_code_nodes;
pub use load_source_code::load_source_code;
pub use parse_source_code::parse_source_code;
pub use track_tree_sitter::track_tree_sitter;