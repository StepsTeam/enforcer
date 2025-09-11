use serde_json::Value as JsonValue; // Aliased serde_json::Value to JsonValue
use std::collections::HashMap;

/// Represents a warning message with an associated level and rule name.
#[derive(Debug, Default, Clone)]
pub struct Warn {
    pub message: String,
    pub level: u8,
    pub rule_name: String,
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
    pub debug_level: u8,
    pub tool_version: String,
    pub tool_url: String,
    pub params: HashMap<String, JsonValue>, // Changed from Option<String> to HashMap
    pub nesting_limit: u8,
    pub traintrack_sarif_rules_str: String,
}

/// The main state structure for the enforcer application.
#[derive(Debug, Default, Clone)]
pub struct Train {
    pub warn: Warn,
    pub watch: Watch,
    pub wreck: Wreck,
    pub tool: Tool,
    pub file_path: Option<std::path::PathBuf>,
    pub data_files_dir: Option<String>,
    pub sarif_rules: JsonValue,
    pub results: JsonValue,
    pub app_logs_dir: Option<std::path::PathBuf>,
    pub sarif_report: JsonValue,
    pub warnings: JsonValue,
    pub function_nodes: JsonValue,
    pub config: JsonValue,
    pub train_data: JsonValue,
}

impl Train {
    /// Creates a new `Train` instance with default values.
    pub fn new() -> Self {
        Self {
            warn: Warn::default(),
            watch: Watch::default(),
            wreck: Wreck::default(),
            tool: Tool {
                params: HashMap::new(), // Initialize params as empty HashMap
                ..Tool::default()
            },
            file_path: None,
            data_files_dir: None,
            sarif_rules: JsonValue::Null,
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
