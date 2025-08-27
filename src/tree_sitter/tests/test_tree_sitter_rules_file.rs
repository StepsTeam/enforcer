use std::fs;
use serde_json::Value;

#[test]
fn test_tree_sitter_sarif_rules_file() {
    let data = fs::read_to_string("tree_sitter/config/tree_sitter_rules.sarif")
        .expect("failed to read SARIF config");
    let sarif: Value = serde_json::from_str(&data)
        .expect("SARIF JSON invalid format");

    let rules = &sarif["runs"][0]["tool"]["driver"]["rules"];
    let rules = rules.as_array().expect("rules is not an array");

    // Check all IDs are unique
    let mut ids = std::collections::HashSet::new();
    for rule in rules {
        let id = rule["id"].as_str().expect("missing id");
        assert!(ids.insert(id), "Duplicate rule id: {}", id);

        let name = rule["name"].as_str().expect("missing name");
        assert!(name.starts_with("TREE_SITTER_"), "Bad rule name: {}", name);
    }

    assert!(!rules.is_empty(), "No rules defined");
}
