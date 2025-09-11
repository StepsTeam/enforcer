use crate::state::{Train};

use crate::sarif::set_sarif_settings::set_sarif_settings;
use crate::cli::configure_cli::configure_cli;
use crate::cli::set_file_path::set_file_path;
use crate::cli::set_app_logs_dir::set_app_logs_dir;
use crate::cli::set_data_files_dir::set_data_files_dir;

pub fn track_cli(mut train: Train) -> Train {
    train = set_app_logs_dir(train);

    train = configure_cli(train);

    train = set_sarif_settings(train);

    train = set_data_files_dir(train);

    train = set_file_path(train);

    train
}
