use crate::Subway;

pub fn finalize_build(subway: Subway) {
    println!("Finalizing build for {} v{}", subway.project_name, subway.version);
    println!("SARIF rules configured: {}", subway.sarif_rules.len());

    if let Some(config_path) = &subway.config_path {
        println!("Config file found at: {}", config_path);
    }

    println!("cargo:rustc-env=ENFORCER_BUILD_COMPLETE=true");
}
