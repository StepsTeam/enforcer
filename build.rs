use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde_json::Value;
use phf_codegen::Map;
use chrono::Local;
use std::process::Command;

fn main() {
    // Paths
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"),
    );
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Create SARIF output directory
    let sarif_output = out_dir.join("sarif");
    fs::create_dir_all(&sarif_output)
        .expect("Build failed: Could not create SARIF output directory.");

    // Write build constants
    let build_constants_path = out_dir.join("build_constants.rs");
    let mut file_constants =
        File::create(&build_constants_path).expect("Failed to create build_constants.rs");
    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writeln!(file_constants, "pub const BUILD_DATE: &str = \"{}\";", datetime)
        .expect("Failed to write build constants");

    // Read SARIF rules
    let sarif_rules_file = manifest_dir.join("src/sarif/tests/test_sarif.json");
    println!("cargo:rerun-if-changed={:?}", sarif_rules_file);
    let sarif_rules_json_str =
        fs::read_to_string(&sarif_rules_file).expect("Failed to read test_sarif.json");
    let sarif_rules_json: Value =
        serde_json::from_str(&sarif_rules_json_str).expect("Failed to parse test_sarif.json");

    let mut rules_map_codegen = Map::new();
    if let Some(runs) = sarif_rules_json["runs"].as_array() {
        if let Some(first_run) = runs.get(0) {
            if let Some(tool) = first_run["tool"].as_object() {
                if let Some(driver) = tool["driver"].as_object() {
                    if let Some(rules) = driver["rules"].as_array() {
                        for rule in rules {
                            if let Some(rule_id) = rule["id"].as_str() {
                                let rule_str =
                                    serde_json::to_string(&rule).expect("Failed to serialize rule");
                                rules_map_codegen.entry(
                                    rule_id,
                                    &format!("\"{}\"", rule_str.replace("\"", "\\\"")),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Write SARIF rules to generated file
    let dest_sarif_rules_path = out_dir.join("static_sarif_rules.rs");
    let generated_code = format!(
        "use phf::Map;\n\npub static SARIF_RULES: Map<&'static str, &'static str> = {};",
        rules_map_codegen.build()
    );
    fs::write(&dest_sarif_rules_path, generated_code)
        .expect("Failed to write static_sarif_rules.rs");

    // Process Tree-sitter language configurations
    let lang_config_path = manifest_dir.join("src/tree_sitter/config/language_configurations.json");
    println!("cargo:rerun-if-changed={:?}", lang_config_path);
    let lang_config_content =
        fs::read_to_string(&lang_config_path).expect("Failed to read language_configurations.json");
    let languages_config: Value =
        serde_json::from_str(&lang_config_content).expect("Failed to parse language_configurations.json");

    let vendor_dir = PathBuf::from("vendor");
    fs::create_dir_all(&vendor_dir).expect("Build failed: Could not create vendor directory.");

    let languages_map = match languages_config.as_object() {
        Some(map) => map,
        None => {
            eprintln!("cargo:warning=language_configurations.json is not a JSON object.");
            panic!("Build failed: Invalid language_configurations.json format.");
        }
    };

    // Clone Tree-sitter grammars
    for (_lang, config) in languages_map.iter() {
        let crate_name = match config.get("crate_name").and_then(Value::as_str) {
            Some(name) => name,
            None => {
                eprintln!("cargo:warning=Missing 'crate_name' in language config. Skipping.");
                continue;
            }
        };
        let dest_path = vendor_dir.join(crate_name);

        if dest_path.exists() && dest_path.join(".git").exists() {
            eprintln!("cargo:warning=Skipping clone for {} (already exists).", crate_name);
            continue;
        }

        let repo_url = format!("https://github.com/tree-sitter/{}.git", crate_name);
        eprintln!("cargo:warning=Cloning {} into {:?}", repo_url, dest_path);

        let output = Command::new("git")
            .args(["clone", "--depth", "1", &repo_url])
            .arg(&dest_path)
            .env("GIT_ASKPASS", "")
            .env("GIT_TERMINAL_PROMPT", "0")
            .output();

        match output {
            Ok(o) if o.status.success() => (),
            Ok(o) => {
                eprintln!("cargo:warning=Git clone failed: {}", String::from_utf8_lossy(&o.stderr));
                continue;
            }
            Err(e) => {
                eprintln!("cargo:warning=Failed to execute git clone: {}", e);
                panic!("Build failed: Git clone failed.");
            }
        }
    }

    // Compile Tree-sitter grammars
    for (lang, config) in languages_map.iter() {
        let crate_name = match config.get("crate_name").and_then(Value::as_str) {
            Some(name) => name,
            None => {
                eprintln!("cargo:warning=Missing 'crate_name' for compilation. Skipping {}.", lang);
                continue;
            }
        };

        let source_path = vendor_dir.join(format!("{}/src/parser.c", crate_name));
        let include_path = vendor_dir.join(format!("{}/src", crate_name));

        if !source_path.exists() {
            eprintln!("cargo:warning=Source file {:?} does not exist. Skipping {}.", source_path, lang);
            continue;
        }

        println!("cargo:rerun-if-changed={:?}", source_path);

        cc::Build::new()
            .file(&source_path)
            .include(&include_path)
            .warnings(false)
            .compile(&format!("tree_sitter_{}", lang));

        eprintln!("cargo:warning=Compiled C grammar for {}", lang);
    }

    println!("cargo:warning=Build process completed successfully.");
}
