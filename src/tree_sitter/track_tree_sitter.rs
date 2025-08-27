use crate::state::Train;
use crate::debug::wreck;

use crate::tree_sitter::configure_tree_sitter::configure_tree_sitter;
use crate::tree_sitter::detect_language::detect_language;
use crate::tree_sitter::load_source_code::load_source_code;
use crate::tree_sitter::parse_source_code::parse_source_code;
use crate::tree_sitter::extract_source_code_nodes::extract_source_code_nodes;

pub fn track_tree_sitter(mut train: Train) -> Train {
    train = configure_tree_sitter(train);

    train = load_source_code(train);

    train = detect_language(train);

    train = parse_source_code(train);

    train = extract_source_code_nodes(train);

    train
}
