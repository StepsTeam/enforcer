// src/sarif/append_train_sarif.rs

use crate::state::{Train, Warn};
use crate::debug::watch;

/// Appends SARIF results to train.results, ensuring no duplicates, and logs actions via watch.
pub fn append_train_sarif(mut train: Train) -> Train {
    // Initial watch log
    train.watch.level = 3;
    train.watch.message = "append_train_sarif: start".to_string();
    train = watch(train);

    // Extract results from SARIF rules
    let results_opt = train
        .sarif_rules
        .get("runs")
        .and_then(|runs| runs.as_array())
        .and_then(|runs_arr| runs_arr.get(0))
        .and_then(|run| run.get("results"))
        .and_then(|res| res.as_array())
        .cloned();

    if let Some(results) = results_opt {
        // Ensure train.results is an array
        if !train.results.is_array() {
            train.results = serde_json::json!([]);
        }

        let train_results = train.results.as_array_mut().unwrap();

        for error in results {
            if let Some(error_message) = error.get("message").and_then(|m| m.as_str()) {
                let duplicate = train_results.iter().any(|existing| {
                    existing.get("message").and_then(|m| m.as_str()) == Some(error_message)
                });

                if !duplicate {
                    train_results.push(error);
                }
            }
        }
    }

    // Clear warnings by assigning a default Warn struct
    train.warn = Warn {
        level: 0,
        rule_name: "".to_string(),
        message: "".to_string(),
    };

    train.watch.level = 5;
    train.watch.message = "SARIF results appended and warnings cleared.".to_string();
    train = watch(train);

    train
}
