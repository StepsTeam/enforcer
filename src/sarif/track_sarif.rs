// src/sarif/track_sarif.rs

// Correct path for Train
use crate::state::{Train};

// Bring in the SARIF functions directly from the sibling modules
use super::configure_sarif::configure_sarif;
use super::set_sarif_settings::set_sarif_settings;
use super::reset_train_warnings::reset_train_warnings;
use super::append_train_sarif::append_train_sarif;
use super::log_sarif_json::log_sarif_json;

/// Orchestrates all SARIF-related transformations on train
pub fn track_sarif(mut train: Train) -> Train {
    train = configure_sarif(train);

    train = set_sarif_settings(train);

    train = reset_train_warnings(train);

    train = append_train_sarif(train);

    train = log_sarif_json(train);

    train
}
