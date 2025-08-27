use std::collections::HashSet;
use std::env;
use std::fs::{self, File}; // fs is correctly imported here for build.rs's direct use
use std::io::Write;
use std::path::PathBuf;
use serde_json::Value;
use phf_codegen::{Map, Set};
use chrono::Local;
use reqwest::blocking::Client;
use std::process::Command;

// This includes the Train struct definition from src/state.rs
// Note: src/state.rs no longer needs its own `use std::fs;` due to this include.
include!("src/state.rs");

fn main() {
    let train = Train::new();

    let out_dir_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    let sarif_output_path = out_dir_path.join("sarif");
    if let Err(e) = fs::create_dir_all(&sarif_output_path) {
        eprintln!("cargo:warning=Failed to create SARIF output directory: {}", e);
        panic!("Build failed: Could not create SARIF output directory.");
    }

    let dest_constants_path = out_dir_path.join("build_constants.rs");
    let mut file_constants = File::create(&dest_constants_path).expect("Failed to create build_constants.rs");
    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writeln!(file_constants, "pub const BUILD_DATE: &str = \"{}\";", datetime).expect("Failed to write build constants");

    println!("cargo:rerun-if-changed=src/oewl/config/oewl_url.txt");
    let oewl_url = train.oewl_url.clone();
    let client = Client::new();

    // Corrected: Use a single match statement to handle the Result and avoid partial moves
    let oewl_content_str: Option<String> = match client.get(&oewl_url).send() {
        Ok(res) => {
            if !res.status().is_success() {
                eprintln!("cargo:warning=Failed to download OEWL content. Status: {}", res.status());
                None
            } else {
                res.text().ok() // .ok() converts Result<String, reqwest::Error> to Option<String>
            }
        }
        Err(e) => {
            eprintln!("cargo:warning=Failed to send HTTP request for OEWL: {}", e);
            None
        }
    };


    let dest_oewl_sets_path = out_dir_path.join("oewl_sets.rs");
    let mut file_oewl_sets = File::create(&dest_oewl_sets_path).expect("Failed to create oewl_sets.rs");
    writeln!(&mut file_oewl_sets, "use phf::phf_set;").unwrap();

    // Corrected: Borrow oewl_content_str here using `ref` to avoid moving the String
    if let Some(ref content) = oewl_content_str {
        let mut nouns: Vec<&str> = Vec::new();
        let mut verbs: Vec<&str> = Vec::new();
        let mut adjectives: Vec<&str> = Vec::new();

        for line in content.lines() { // content is &String, lines() takes &self, which is compatible
            if line.starts_with("!n ") {
                nouns.push(line.trim_start_matches("!n "));
                continue;
            }
            if line.starts_with("!v ") {
                verbs.push(line.trim_start_matches("!v "));
                continue;
            }
            if line.starts_with("!a ") {
                adjectives.push(line.trim_start_matches("!a "));
                continue;
            }
        }

        let mut nouns_set = Set::new();
        for n in nouns {
            nouns_set.entry(n);
        }
        writeln!(&mut file_oewl_sets, "pub static OEWL_NOUNS: phf::Set<&'static str> = {};", nouns_set.build()).expect("Failed to write OEWL nouns");

        let mut verbs_set = Set::new();
        for v in verbs {
            verbs_set.entry(v);
        }
        writeln!(&mut file_oewl_sets, "pub static OEWL_VERBS: phf::Set<&'static str> = {};", verbs_set.build()).expect("Failed to write OEWL verbs");

        let mut adjectives_set = Set::new();
        for a in adjectives {
            adjectives_set.entry(a);
        }
        writeln!(&mut file_oewl_sets, "pub static OEWL_ADJECTIVES: phf::Set<&'static str> = {};", adjectives_set.build()).expect("Failed to write OEWL adjectives");
    }

    // This check is now safe because oewl_content_str was only borrowed above, not moved.
    // So, .is_none() still accurately reflects its state.
    if oewl_content_str.is_none() {
        writeln!(&mut file_oewl_sets, "pub static OEWL_NOUNS: phf::Set<&'static str> = phf_set! {{}};").expect("Failed to write empty OEWL nouns");
        writeln!(&mut file_oewl_sets, "pub static OEWL_VERBS: phf::Set<&'static str> = phf_set! {{}};").expect("Failed to write empty OEWL verbs");
        writeln!(&mut file_oewl_sets, "pub static OEWL_ADJECTIVES: phf::Set<&'static str> = phf_set! {{}};").expect("Failed to write empty OEWL adjectives");
    }

    println!("cargo:rerun-if-changed=src/traintrack/config/naming_words.txt");
    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let naming_words_path = manifest_dir_path.join("src/traintrack/config/naming_words.txt");
    
    let naming_words_content = fs::read_to_string(&naming_words_path)
        .expect(&format!("Failed to read naming_words.txt from {:?}", naming_words_path));
    
    let words: HashSet<String> = naming_words_content
        .lines()
        .map(|s| s.to_lowercase())
        .collect();

    let dest_naming_words_path = out_dir_path.join("naming_words_arrays.rs");
    let mut file_naming_words = File::create(&dest_naming_words_path)
        .expect("Failed to create naming_words_arrays.rs");
    writeln!(&mut file_naming_words, "use phf::phf_set;").unwrap();
    let mut builder = Set::new();
    for word in &words {
        builder.entry(word.as_str());
    }
    writeln!(
        &mut file_naming_words,
        "pub static NAMING_WORDS: phf::Set<&'static str> = {};",
        builder.build()
    ).expect("Failed to write naming words array");


    let sarif_rules_file_path = manifest_dir_path.join("tests/test_sarif.json");
    println!("cargo:rerun-if-changed={:?}", sarif_rules_file_path);
    let sarif_rules_json_str = fs::read_to_string(&sarif_rules_file_path)
        .expect(&format!("Failed to read test_sarif.json from {:?}", sarif_rules_file_path));

    let sarif_rules_json: Value = serde_json::from_str(&sarif_rules_json_str)
        .expect("Failed to parse test_sarif.json");

    let mut rules_map_codegen = Map::new();

    if let Some(runs_array) = sarif_rules_json["runs"].as_array() {
        if let Some(first_run) = runs_array.get(0) {
            if let Some(tool) = first_run["tool"].as_object() {
                if let Some(driver) = tool["driver"].as_object() {
                    if let Some(rules) = driver["rules"].as_array() {
                        for rule in rules {
                            if let Some(rule_id) = rule["id"].as_str() {
                                let rule_str = serde_json::to_string(&rule).expect("Failed to serialize rule");
                                rules_map_codegen.entry(rule_id, &format!("\"{}\"", rule_str.replace("\"", "\\\"")));
                            }
                        }
                    }
                }
            }
        }
    }

    let dest_sarif_rules_path = out_dir_path.join("static_sarif_rules.rs");
    let _file_sarif_rules = File::create(&dest_sarif_rules_path).expect("Failed to create static_sarif_rules.rs");
    let generated_code = format!(
        "use phf::Map;\n\npub static SARIF_RULES: Map<&'static str, &'static str> = {};",
        rules_map_codegen.build()
    );
    fs::write(&dest_sarif_rules_path, generated_code).expect("Failed to write to generated static_sarif_rules.rs");


    println!("cargo:rerun-if-changed=src/tree_sitter/config/language_configurations.json");
    let lang_config_path = manifest_dir_path.join("src/tree_sitter/config/language_configurations.json");
    let json_content_lang = fs::read_to_string(&lang_config_path)
        .expect(&format!("Failed to read language_configurations.json from {:?}", lang_config_path));
    let languages_config: Value = serde_json::from_str(&json_content_lang)
        .expect("Failed to parse language_configurations.json");

    let vendor_dir = PathBuf::from("vendor");
    if let Err(e) = fs::create_dir_all(&vendor_dir) {
        eprintln!("cargo:warning=Failed to create vendor directory: {}", e);
        panic!("Build failed: Could not create vendor directory.");
    }

    let Some(map) = languages_config.as_object() else {
        eprintln!("cargo:warning=language_configurations.json is not a JSON object. Skipping grammar acquisition.");
        panic!("Build failed: Invalid language_configurations.json format.");
    };

    for (_lang, config) in map.iter() {
        let Some(crate_name) = config.get("crate_name").and_then(Value::as_str) else {
            eprintln!("cargo:warning=Missing 'crate_name' in language config. Skipping.");
            continue;
        };
        let dest_path = vendor_dir.join(crate_name);

        if dest_path.exists() && dest_path.join(".git").exists() {
            eprintln!("cargo:warning=Skipping clone for {} as it already exists and is a git repo.", crate_name);
            continue;
        }
        
        let repo_url = format!("https://github.com/tree-sitter/{}.git", crate_name);
        eprintln!("cargo:warning=Cloning Tree-sitter grammar: {} into {:?}", repo_url, dest_path);
        
        let output = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(&repo_url)
            .arg(&dest_path)
            .env("GIT_ASKPASS", "")
            .env("GIT_TERMINAL_PROMPT", "0")
            .output();

        let cmd_output = match output {
            Ok(o) => o,
            Err(e) => {
                eprintln!("cargo:warning=Failed to execute git clone for {}: {}", repo_url, e);
                panic!("Build failed: Git clone command execution failed.");
            }
        };

        if !cmd_output.status.success() {
            eprintln!("cargo:warning=Git clone failed for {}. Stderr: {}", repo_url, String::from_utf8_lossy(&cmd_output.stderr));
            continue; 
        }
    }

    let Some(map_compile) = languages_config.as_object() else {
        eprintln!("cargo:warning=language_configurations.json is not a JSON object for compilation. Skipping grammar compilation.");
        panic!("Build failed: Invalid language_configurations.json format for compilation.");
    };

    for (lang, config) in map_compile.iter() {
        let Some(crate_name) = config.get("crate_name").and_then(Value::as_str) else {
            eprintln!("cargo:warning=Missing 'crate_name' in language config for compilation. Skipping.");
            continue;
        };
        let source_path = vendor_dir.join(format!("{}/src/parser.c", crate_name));
        let header_path = vendor_dir.join(format!("{}/src", crate_name));
        
        if !source_path.exists() {
            eprintln!("cargo:warning=Source file {:?} for language {} does not exist. Skipping compilation.", source_path, lang);
            continue;
        }

        println!("cargo:rerun-if-changed={:?}", source_path);
        
        let mut build = cc::Build::new();
        build.file(&source_path);
        build.include(&header_path);
        build.warnings(false);

        eprintln!("cargo:warning=Compiling C grammar for {}", lang);
        build.compile(&format!("tree_sitter_{}", lang));
    }

    println!("cargo:warning=Build process completed successfully.");
}
