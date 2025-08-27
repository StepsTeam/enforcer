// src/sarif/mod.rs

// Include the generated static SARIF rules file here.
// This makes STATIC_SARIF_RULES available within the sarif module's scope.
include!(concat!(env!("OUT_DIR"), "/static_sarif_rules.rs"));

// The `include!` macro already places STATIC_SARIF_RULES into this module's scope.
// No need to `pub use` it again, as that causes a redefinition error (E0255).
// Other modules will now access it via `crate::sarif::STATIC_SARIF_RULES`.
// Removed: pub use STATIC_SARIF_RULES;

// Declare the sub-modules within the 'sarif' directory.
pub mod track_sarif;
pub mod append_train_sarif;
pub mod log_sarif_json;
pub mod reset_train_warnings;
pub mod set_sarif_settings;
pub mod get_tool_sarif_rules;
pub mod configure_sarif; // Assuming this module exists and is used by track_sarif

// Re-export the public functions from these modules.
pub use track_sarif::track_sarif;
pub use append_train_sarif::append_train_sarif;
pub use log_sarif_json::log_sarif_json;
pub use reset_train_warnings::reset_train_warnings;
pub use set_sarif_settings::set_sarif_settings;
pub use get_tool_sarif_rules::get_tool_sarif_rules;
pub use configure_sarif::configure_sarif;
