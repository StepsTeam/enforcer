// build/set_sarif_rules_constant.rs

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use crate::Subway;

pub fn set_sarif_rules_constant(mut subway: Subway) -> Subway {
    println!("Processing SARIF rules...");

    if subway.project_name.is_empty() {
        subway.project_name = "Enforcer".to_string();
    }
    if subway.version.is_empty() {
        subway.version = env!("CARGO_PKG_VERSION").to_string();
    }

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/");

    // Placeholder for rust_sitter_tool::build_parsers call
    println!("(Simulated) Building parsers with rust_sitter_tool");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = PathBuf::from(&out_dir).join("sarif_rules.rs");
    let mut file = File::create(&dest_path).expect("Failed to create sarif_rules.rs");

    writeln!(file, "use phf::phf_map;").expect("Failed to write header");
    writeln!(
        file,
        "pub static SARIF_RULES: phf::Map<&'static str, &'static str> = phf_map! {{"
    )
    .expect("Failed to write map start");

    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = crate_root.join("src");

    let mut found_rules = Vec::new();

    for entry in fs::read_dir(&src_dir).expect("Failed to read src directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(tool_name) = path.file_name().and_then(|n| n.to_str()) {
                    let sarif_path = path.join("config").join(format!("{}_rules.sarif", tool_name));
                    if sarif_path.exists() {
                        let abs_path = sarif_path.canonicalize().expect("Failed to canonicalize SARIF path");
                        writeln!(
                            file,
                            r#"    "{}" => include_str!(r"{}"),"#,
                            tool_name,
                            abs_path.display()
                        )
                        .expect("Failed to write entry");
                        println!("cargo:rerun-if-changed={}", abs_path.display());
                        found_rules.push(tool_name.to_string());
                    }
                }
            }
        }
    }

    writeln!(file, "}};").expect("Failed to write map end");

    subway.sarif_rules = found_rules;

    subway
}
