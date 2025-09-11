// build_helpers/mod.rs

// Declare the individual module files within this directory.
pub mod set_sarif_rules_constant;
// Removed: pub mod set_config_paths;
pub mod generate_build_constants;
pub mod finalize_build;
pub mod collect_sarif_rules;
// Removed: pub mod create_naming_words_arrays;

// Add `serde_json::Value` import for the new field in Subway
use serde_json::Value;

// Define your main struct that will be processed through the pipeline.
#[derive(Debug, Clone)]
pub struct Subway {
    pub project_name: String,
    pub version: String,
    pub sarif_rules: Vec<String>,
    pub config_path: Option<String>,
    pub combined_sarif_rules: Option<Value>, // This field is crucial for collect_sarif_rules
}

impl Subway {
    pub fn new() -> Self {
        Self {
            project_name: "Enforcer".to_string(),
            version: "0.1.0".to_string(),
            sarif_rules: Vec::new(),
            config_path: None,
            combined_sarif_rules: None,
        }
    }
}

// Re-export the functions from their respective files.
pub use set_sarif_rules_constant::set_sarif_rules_constant;
// Removed: pub use set_config_paths::set_config_paths;
pub use generate_build_constants::generate_build_constants;
pub use finalize_build::finalize_build;
pub use collect_sarif_rules::collect_sarif_rules;
