use std::env;
use std::path::Path;
use crate::Subway;

pub fn set_config_paths(mut subway: Subway) -> Subway {
    println!("Setting configuration paths...");

    let config_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config_path = Path::new(&config_dir).join("config").join("enforcer.toml");

    if config_path.exists() {
        subway.config_path = Some(config_path.to_string_lossy().to_string());
        println!("cargo:rustc-env=ENFORCER_CONFIG_PATH={}", config_path.display());
    }

    println!("cargo:warning=Configuration file not found at {:?}", config_path);

    subway
}
