use std::env;
use std::fs;
use std::path::Path;
use crate::Subway;

pub fn generate_build_constants(subway: Subway) -> Subway {
    println!("Generating build constants...");

    let build_info = format!(
        r#"
#[derive(Debug)]
pub struct BuildInfo {{
    pub name: &'static str,
    pub version: &'static str,
    pub sarif_rules_count: usize,
    pub has_config: bool,
}}
pub const BUILD_INFO: BuildInfo = BuildInfo {{
    name: "{name}",
    version: "{version}",
    sarif_rules_count: {rules_count},
    has_config: {has_config},
}};
"#,
        name = subway.project_name,
        version = subway.version,
        rules_count = subway.sarif_rules.len(),
        has_config = subway.config_path.is_some(),
    );

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    fs::write(&dest_path, build_info).expect("Failed to write build_info.rs");

    subway
}
