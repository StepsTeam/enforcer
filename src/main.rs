// src/main.rs
use clap::Parser;

// Removed: pub use SARIF_RULES; // This was causing a conflict

mod cli;
mod oewl;
mod sarif;
mod state;
mod traintrack;
mod tree_sitter_utils; // Renamed to tree_sitter_utils to avoid conflict with tree_sitter crate

// Add a test module -- this is ignored when not running tests.
#[cfg(test)]
mod tests;

fn main() {
    let mut train = state::Train::new();

    // Track CLI arguments and apply to train
    train = cli::track_cli(train);

    // Acquire OEWL rules
    oewl::acquire_oewl(&mut train);

    // Configure SARIF reporting
    sarif::configure_sarif(&mut train);

    // Track Tree-sitter details
    tree_sitter_utils::track_tree_sitter(&mut train);

    // Track Traintrack specific rules
    traintrack::track_traintrack(&mut train);

    // After all checks, log the SARIF report
    sarif::log_sarif_json(&mut train);

    println!("Final Train State: {:?}", train);
}