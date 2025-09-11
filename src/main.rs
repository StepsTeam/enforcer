// /opt/enforcer/src/main.rs

mod debug;
mod sarif;
mod tree_sitter;
mod traintrack;
mod state;

use crate::sarif::configure_sarif::configure_sarif;
use crate::sarif::log_sarif_json::log_sarif_json;
use crate::tree_sitter::track_tree_sitter::track_tree_sitter;
use crate::traintrack::track_traintrack::track_traintrack;
use crate::state::Train;

fn main() {
    // Initialize Train
    let train = Train::new();

    // Run SARIF configuration
    let train = configure_sarif(train);

    // Run Tree-sitter analysis
    let train = track_tree_sitter(train);

    // Run TrainTrack enforcement
    let train = track_traintrack(train);

    // Log SARIF JSON
    let train = log_sarif_json(train);

    println!("Pipeline finished. Train: {:?}", train);
}
