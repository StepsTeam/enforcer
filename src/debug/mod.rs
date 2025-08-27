pub mod warn;
pub mod watch;
pub mod wreck;
pub mod configure_debug;

pub use configure_debug::configure_debug; // Re-export configure_debug for main.rs
// Removed: pub use warn::warn; // Was unused, causing a warning
// Add other `pub use` statements here if any functions from `warn`, `watch`, or `wreck`
// need to be accessible directly via `debug::function_name` from outside the `debug` module.
// For now, other modules directly import them via their full path (e.g., `debug::watch::watch`).
