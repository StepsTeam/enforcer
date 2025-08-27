// Removed top-level imports for std::fs, std::io, and std::path::PathBuf
// These will be imported locally within the `Train::new()` function where needed.
use serde_json::Value as JsonValue; // Aliased serde_json::Value to JsonValue

/// Represents a warning message with an associated level and rule name.
#[derive(Debug, Default, Clone)]
pub struct Warn {
    pub message: String,
    pub level: u8,
    pub rule_name: String, // Added rule_name field
}

/// Represents a watch/logging configuration.
#[derive(Debug, Default, Clone)]
pub struct Watch {
    pub level: u8,
    pub message: String,
}

/// Represents a critical error or "wreck" state with a message.
#[derive(Debug, Default, Clone)]
pub struct Wreck {
    pub message: String,
}

/// Represents a tool with its configuration.
#[derive(Debug, Default, Clone)]
pub struct Tool {
    pub tool_name: String,
    pub debug_level: u8,                   // Added debug_level field
    pub tool_version: String,              // Added tool_version field
    pub pub_url: String,                  // Added tool_url field, renamed to pub_url
    pub params: Option<String>,            // Added params field
    pub nesting_limit: u8,                 // Added nesting_limit field
    pub traintrack_sarif_rules_str: String, // Added traintrack_sarif_rules_str field
}

/// The main state structure for the enforcer application.
#[derive(Debug, Default, Clone)]
pub struct Train {
    pub oewl_url: String,
    pub warn: Warn, // Directly stores Warn, not an Option<Warn>
    pub watch: Watch,
    pub wreck: Wreck,
    pub tool: Tool,
    pub file_path: Option<std::path::PathBuf>, // Added file_path field
    pub data_files_dir: Option<String>,       // Added data_files_dir field
    pub sarif_rules: JsonValue,                   // Changed to JsonValue
    pub results: JsonValue,                       // Changed to JsonValue
    pub app_logs_dir: Option<std::path::PathBuf>, // Added app_logs_dir field
    pub sarif_report: JsonValue,                  // Changed to JsonValue
    pub warnings: JsonValue,                      // Changed to JsonValue
    pub function_nodes: JsonValue,                // Changed to JsonValue
    pub config: JsonValue,                        // Changed to JsonValue
    pub train_data: JsonValue,                    // Changed to JsonValue
}

impl Train {
    /// Creates a new `Train` instance with default values.
    pub fn new() -> Self {
        // Local imports for `fs` and `PathBuf` within this function's scope.
        use std::fs;
        use std::path::PathBuf;

        let oewl_url_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("oewl")
            .join("config")
            .join("oewl_url.txt");

        let oewl_url = match fs::read_to_string(&oewl_url_path) {
            Ok(url) => url.trim().to_string(),
            Err(e) => {
                panic!(
                    "Could not read oewl_url.txt from {:?}. Ensure the file exists at this path relative to the project root: {}",
                    oewl_url_path, e
                );
            }
        };

        Self {
            oewl_url,
            warn: Warn::default(), // Initialize with default Warn
            watch: Watch::default(),
            wreck: Wreck::default(),
            tool: Tool::default(),
            // Initialize new fields with appropriate defaults
            file_path: None,
            data_files_dir: None,
            sarif_rules: JsonValue::Null, // Or serde_json::json!({}) for an empty object
            results: JsonValue::Array(vec![]),
            app_logs_dir: None,
            sarif_report: JsonValue::Null,
            warnings: JsonValue::Array(vec![]),
            function_nodes: JsonValue::Array(vec![]),
            config: JsonValue::Null,
            train_data: JsonValue::Array(vec![]),
        }
    }
}
