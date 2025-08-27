use crate::state::Train;
use crate::debug::watch::watch;
use serde_json::Value; 

/// Append all valid SARIF-like error objects in train[flaws] to train[sarif]
/// Removes duplicate train[flaws] to make the SARIF JSON output more concise
/// Removes train[flaws] finally to help maintain the size of the train array
pub fn append_train_sarif(mut train: Train) -> Train {
    train.watch.level = 3;
    train.watch.message = "append_train_sarif:".to_string();
    train = watch(train);

    // Read from train.sarif_rules (which is of type Value)
    let results_opt = train
        .sarif_rules
        .get("runs")
        .and_then(|runs| runs.as_array())
        .and_then(|runs_arr| runs_arr.get(0))
        .and_then(|run| run.get("results"))
        .and_then(|res| res.as_array())
        .cloned();

    if let Some(results) = results_opt {
        // This part assumes 'results' is a direct field on 'Train'
        // If not, it needs to be added to src/state.rs
        if !train.results.is_array() { // Assumes train.results exists and is a Value
            train.results = serde_json::json!([]); // Initialize if not an array
        }

        let train_results = train
            .results
            .as_array_mut()
            .expect("train.results must be an array");

        for error in results {
            if !train_results.iter().any(|existing| {
                existing.get("message")
                    == error
                        .get("message")
                        .and_then(|m| m.as_str())
                        .map(|s| Value::String(s.to_string()))
                        .as_ref()
            }) {
                train_results.push(error.clone());
            }
        }
    }

    train.watch.level = 5;
    train.watch.message = "SARIF results appended to train.".to_string();
    train = watch(train);

    train
}
