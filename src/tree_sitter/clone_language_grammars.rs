use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use std::process::Command;

use crate::state::{Train, Warn};
use crate::debug::watch::watch;
use crate::debug::wreck::wreck;

/// Clones Tree-sitter language grammars based on the configuration stored in `train.language_configurations`.
/// The grammars are cloned into the directory specified by `train.vendor_dir`.
/// This function now takes and returns only the `Train` struct.
pub fn clone_language_grammars(mut train: Train) -> Train {
    if !train.wreck.message.is_empty() {
        return train;
    }

    train.watch.level = 3;
    train.watch.message = "clone_language_grammars: Starting grammar cloning process.".to_string();
    train = watch(train);

    // Ensure language_configurations is a JSON object.
    let Some(map) = train.language_configurations.as_object() else {
        train.wreck.message = "CLONE_GRAMMAR_INVALID_CONFIG: language_configurations field in Train is not a JSON object. Skipping grammar cloning.".to_string();
        return wreck(train);
    };

    let vendor_dir = &train.vendor_dir; // Access vendor_dir from the train struct

    for (_lang, config) in map.iter() {
        // Check for wreck inside the loop to allow graceful exit if a previous iteration caused a wreck.
        if !train.wreck.message.is_empty() {
            train.watch.message = format!("clone_language_grammars: Wrecked, skipping further cloning. Current wreck: {}", train.wreck.message);
            train = watch(train);
            return train; // Return early if already wrecked
        }

        // Extract 'crate_name'
        let Some(crate_name) = config.get("crate_name").and_then(Value::as_str) else {
            train.warn_message = Some(Warn {
                rule_name: "CLONE_GRAMMAR_MISSING_CRATE_NAME".to_string(),
                message: "Missing 'crate_name' for a language configuration. Skipping clone.".to_string(),
            });
            train.watch.message = format!("clone_language_grammars: Warned: {:?}", train.warn_message);
            train = watch(train);
            continue; // Move to the next language
        };
        let dest_path = vendor_dir.join(crate_name);

        // Check if the directory exists and is a valid git repository.
        if dest_path.exists() && dest_path.join(".git").exists() {
            train.watch.message = format!("Skipping clone for {} as it already exists and is a git repo.", crate_name);
            train = watch(train);
            continue; // Move to the next language
        }

        // Extract 'repository_url'
        let Some(repo_url) = config.get("repository_url").and_then(Value::as_str) else {
            train.warn_message = Some(Warn {
                rule_name: "CLONE_GRAMMAR_MISSING_REPO_URL".to_string(),
                message: format!("Missing 'repository_url' for language {}. Skipping clone.", crate_name),
            });
            train.watch.message = format!("clone_language_grammars: Warned: {:?}", train.warn_message);
            train = watch(train);
            continue; // Move to the next language
        };

        let branch = config.get("branch")
                       .and_then(Value::as_str)
                       .unwrap_or("master"); // Default to 'master' if branch is not specified

        train.watch.level = 5;
        train.watch.message = format!("Cloning Tree-sitter grammar: {} (branch: {}) into {:?}", repo_url, branch, dest_path);
        train = watch(train);

        let output = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("--branch")
            .arg(branch)
            .arg(repo_url)
            .arg(&dest_path)
            .env("GIT_ASKPASS", "") // Disable Git asking for password
            .env("GIT_TERMINAL_PROMPT", "0") // Disable terminal prompts
            .output();

        let cmd_output = match output {
            Ok(o) => o,
            Err(e) => {
                train.wreck.message = format!("CLONE_GRAMMAR_GIT_EXEC_FAILED: Failed to execute git clone for {}: {}", repo_url, e);
                return wreck(train); // Early exit for execution error
            }
        };

        if !cmd_output.status().success() {
            train.warn_message = Some(Warn {
                rule_name: "CLONE_GRAMMAR_GIT_FAILED".to_string(),
                message: format!("Git clone failed for {}. Stderr: {}", repo_url, String::from_utf8_lossy(&cmd_output.stderr)),
            });
            train.watch.message = format!("clone_language_grammars: Warned: {:?}", train.warn_message);
            train = watch(train);
            continue; // Continue to the next language if clone failed
        }

        // If we reach here, it means cmd_output.status().success() was true
        train.watch.message = format!("Successfully cloned {} to {:?}", repo_url, dest_path);
        train = watch(train);
    }
    train
}
