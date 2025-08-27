#!/usr/bin/env bash
set -euo pipefail

function compile_enforcer {
    clear

    cd /opt/enforcer || {
        echo "Failed to enter /opt/enforcer"
        exit 1
    }

    cargo clean

    cargo build --verbose

    cargo run -- --file-path src/cli/configure_cli.rs
}

# Run the function when the script is executed
compile_enforcer
