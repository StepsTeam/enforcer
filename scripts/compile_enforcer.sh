#!/usr/bin/env bash
set -euo pipefail

function compile_enforcer {
    clear

    cd /opt/enforcer || {
        echo "Failed to enter /opt/enforcer"
        exit 1
    }

    # Ignore the files in /opt/enforcer/target/ and vendor/ directories
    # tree -I 'target|vendor' /opt/enforcer > docs/directory_structure.txt
    tree -I 'target' /opt/enforcer > docs/directory_structure.txt

    cargo clean

    cargo build --verbose

    cargo run -- --file-path src/cli/configure_cli.rs
}

# Run the function when the script is executed
compile_enforcer
