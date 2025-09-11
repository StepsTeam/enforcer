// build_helpers/collect_sarif_rules.rs

use std::env;
use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use super::Subway;
use std::collections::VecDeque;

pub fn collect_sarif_rules(mut subway: Subway) -> Subway {
    println!("Building: Collecting SARIF rules for static embedding...");

    // Initialize combined_sarif_rules with a base structure and an empty "runs" array.
    // This `runs` array will be populated by appending entire `run` objects from individual SARIF files.
    subway.combined_sarif_rules = Some(json!({
        "version": "2.1.0",
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "runs": [] // Start with an empty runs array
    }));

    // Get a mutable reference to the "runs" array in the combined SARIF.
    let combined_runs_array = subway.combined_sarif_rules
                            .as_mut()
                            .expect("combined_sarif_rules was not initialized correctly")
                            .get_mut("runs").expect("Missing 'runs' in combined_rules JSON structure")
                            .as_array_mut().expect("'runs' is not an array");


    let current_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let src_path = PathBuf::from(current_dir).join("src");

    let mut directories_to_visit: VecDeque<PathBuf> = VecDeque::new();
    directories_to_visit.push_back(src_path);

    while let Some(current_dir_path) = directories_to_visit.pop_front() {

        let entries = match fs::read_dir(&current_dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Warning: Could not read directory {:?}: {}", &current_dir_path, e);
                continue;
            }
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_dir() {
                directories_to_visit.push_back(path);
                continue;
            }

            // Only process .sarif files
            if path.extension().map_or(false, |ext| ext != "sarif") {
                continue;
            }

            println!("Found SARIF file: {:?}", path);

            // Read file content
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Warning: Could not read file {:?}: {}", path, e);
                    continue;
                }
            };

            // Parse file content as JSON
            let sarif_json: Value = match serde_json::from_str(&content) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Warning: Could not parse {:?} as valid JSON: {}", path, e);
                    continue;
                }
            };

            // Extract the "runs" array from the parsed SARIF file.
            // This is the key change: we're taking the *entire* run objects, not just rules.
            let file_runs = match sarif_json.get("runs")
                .and_then(|r| r.as_array())
            {
                Some(runs) => runs,
                None => {
                    eprintln!("Warning: No 'runs' array found in SARIF file: {:?}", path);
                    continue;
                }
            };

            // Extend the combined_runs_array with the runs from the current file.
            for run_entry in file_runs {
                combined_runs_array.push(run_entry.clone());
            }
        }
    }

    let final_combined_rules = subway.combined_sarif_rules
                                     .as_ref()
                                     .expect("combined_sarif_rules should have been populated");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest_path = out_dir.join("static_sarif_rules.rs");

    let file_content = format!(
        "pub static STATIC_SARIF_RULES: &str = r#\"{}\"#;",
        final_combined_rules.to_string()
    );

    fs::write(&dest_path, file_content).expect("Failed to write static_sarif_rules.rs");
    println!("Generated static SARIF rules file at: {:?}", dest_path);

    // Tell Cargo to re-run the build script if any of these directories change.
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=src/cli/config/");
    println!("cargo:rerun-if-changed=src/debug/config/");
    println!("cargo:rerun-if-changed=src/sarif/config/");
    println!("cargo:rerun-if-changed=src/traintrack/config/");

    subway
}
