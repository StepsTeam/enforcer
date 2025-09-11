// build_helpers/collect_naming_words.rs

use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::env;

use super::Subway; // Import Subway struct from build_helpers/mod.rs

/// collect_naming_words()
/// - Reads words from `verbs.txt`, `adjectives.txt`, and `nouns.txt` located in `subway.definitions_dir`.
/// - Extracts the term before the first colon from each line.
/// - Populates `subway.function_verbs`, `subway.function_adjectives`, and `subway.function_nouns`.
/// - Returns the updated Subway struct.
fn collect_naming_words(mut subway: Subway) -> Subway {
    // Ensure definitions_dir is set, otherwise return immediately.
    let definitions_dir = match subway.definitions_dir.clone() {
        Some(dir) => dir,
        None => {
            eprintln!("Error: definitions_dir not set in Subway. Cannot collect word definitions.");
            return subway;
        }
    };

    let files_to_read = [
        ("verbs", definitions_dir.join("verbs.txt")),
        ("adjectives", definitions_dir.join("adjectives.txt")),
        ("nouns", definitions_dir.join("nouns.txt")),
    ];

    for (word_type, file_path) in files_to_read.iter() {
        let result = fs::File::open(file_path);
        let words_vec = match result {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                let mut collected_words = Vec::new();
                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("Warning: Error reading line from {:?}: {}", file_path, e);
                            continue;
                        }
                    };
                    if let Some(word) = line.split(':').next() {
                        let trimmed_word = word.trim();
                        if !trimmed_word.is_empty() {
                            collected_words.push(trimmed_word.to_string());
                        }
                    }
                }
                collected_words
            },
            Err(e) => {
                eprintln!("Warning: Could not read {} from {:?}: {}", word_type, file_path, e);
                Vec::new() // Return an empty vector on file open error
            }
        };

        match *word_type {
            "verbs" => subway.function_verbs = Some(words_vec),
            "adjectives" => subway.function_adjectives = Some(words_vec),
            "nouns" => subway.function_nouns = Some(words_vec),
            _ => eprintln!("Error: Unknown word_type '{}' in collect_naming_words loop.", word_type),
        }

        // Tell Cargo to re-run if any of the definition files change
        println!("cargo:rerun-if-changed={}", file_path.display());
    }

    subway
}
