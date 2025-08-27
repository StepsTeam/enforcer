#!/usr/bin/env bash

# sync_tree_sitter_configs.sh
# This script reads src/tree_sitter/config/language_configurations.json
# and GENERATES AND UPDATES the corresponding Cargo.toml dependencies and
# src/tree_sitter/detect_language.rs imports and match arms IN PLACE.
#
# Usage:
#   Call this script directly (e.g., ./scripts/sync_tree_sitter_configs.sh)

set -euo pipefail # Exit immediately if a command exits with a non-zero status.

# Ensure jq is installed for JSON parsing
if ! command -v jq &> /dev/null; then
    echo "Error: jq is not installed. Please install it using: sudo apt-get install -y jq"
    exit 1
fi

# Define paths relative to the project root
LANGUAGE_CONFIG_FILE="../src/tree_sitter/config/language_configurations.json"
CARGO_TOML_PATH="../Cargo.toml"
DETECT_LANGUAGE_RS_PATH="../src/tree_sitter/detect_language.rs"

# --- TAGS FOR IN-PLACE REPLACEMENT ---
CARGO_DEPS_START="# --- GENERATED TREE-SITTER DEPENDENCIES START ---"
CARGO_DEPS_END="# --- GENERATED TREE-SITTER DEPENDENCIES END ---"

DETECT_LANG_IMPORTS_START="// --- GENERATED TREE-SITTER IMPORTS START ---"
DETECT_LANG_IMPORTS_END="// --- GENERATED TREE-SITTER IMPORTS END ---"

DETECT_LANG_MATCH_START="    // --- GENERATED TREE-SITTER MATCH ARMS START ---"
DETECT_LANG_MATCH_END="    // --- GENERATED TREE-SITTER MATCH ARMS END ---"

# Helper function to replace content between specific tags in a file
# Arguments: file_path, start_tag, end_tag, new_content
function replace_section {
    local file="$1"
    local start_tag="$2"
    local end_tag="$3"
    local new_content="$4"

    local temp_file
    temp_file=$(mktemp)

    local in_section=0
    while IFS= read -r line; do
        if [[ "$line" == *"$start_tag"* ]]; then
            echo "$line" >> "$temp_file" # Write start tag
            echo -e "$new_content" >> "$temp_file" # Write new content
            in_section=1
        elif [[ "$line" == *"$end_tag"* ]]; then
            in_section=0
            echo "$line" >> "$temp_file" # Write end tag
        elif [[ "$in_section" -eq 0 ]]; then
            echo "$line" >> "$temp_file" # Write lines outside the section
        fi
    done < "$file"

    mv "$temp_file" "$file"
    echo "  Updated $file section: '$start_tag' to '$end_tag'"
}


# --- Main function for synchronization ---
function sync_tree_sitter_configurations {
    echo "${FUNCNAME[0]}(): Generating and updating Tree-sitter configurations..."

    if [[ ! -f "$LANGUAGE_CONFIG_FILE" ]]; then
        echo "Error: Language configuration file not found at $LANGUAGE_CONFIG_FILE"
        exit 1
    fi

    # Read and parse the JSON configuration
    local lang_configs_json
    lang_configs_json=$(cat "$LANGUAGE_CONFIG_FILE")
    if [[ $? -ne 0 ]]; then
        echo "Error: Failed to read $LANGUAGE_CONFIG_FILE"
        exit 1
    fi

    # --- Generate Cargo.toml dependencies content ---
    local cargo_deps_output
    cargo_deps_output=$(echo "$lang_configs_json" | jq -r '
        to_entries | sort_by(.key)[] |
        "\(.value.crate_name) = \"\(.value.crate_version)\""
    ' | sed '/^$/d') # Remove empty lines

    # --- Generate detect_language.rs imports content ---
    local detect_lang_imports_output
    detect_lang_imports_output=$(echo "$lang_configs_json" | jq -r '
        to_entries | sort_by(.key)[] |
        "use \(.value.crate_name | gsub("-"; "_"))::LANGUAGE;"
    ' | sed '/^$/d')

    # --- Generate detect_language.rs match arms content ---
    local detect_lang_match_arms_output
    detect_lang_match_arms_output=$(echo "$lang_configs_json" | jq -r '
        to_entries | sort_by(.key)[] |
        "        \"\(.value.crate_name)\" => \(.value.crate_name | gsub("-"; "_"))::LANGUAGE.into(),"
    ' | sed '/^$/d')

    # --- Perform in-place file updates ---
    replace_section "$CARGO_TOML_PATH" "$CARGO_DEPS_START" "$CARGO_DEPS_END" "$cargo_deps_output"
    replace_section "$DETECT_LANGUAGE_RS_PATH" "$DETECT_LANG_IMPORTS_START" "$DETECT_LANG_IMPORTS_END" "$detect_lang_imports_output"
    replace_section "$DETECT_LANGUAGE_RS_PATH" "$DETECT_LANG_MATCH_START" "$DETECT_LANG_MATCH_END" "$detect_lang_match_arms_output"

    echo "  Tree-sitter configuration synchronization complete."
}

# Call the function directly when the script is executed
sync_tree_sitter_configurations
