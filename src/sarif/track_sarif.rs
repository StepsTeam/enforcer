use crate::state::Train;

use crate::sarif::configure_sarif::configure_sarif;
use crate::sarif::set_sarif_settings::set_sarif_settings;
use crate::sarif::reset_train_warnings::reset_train_warnings;
use crate::sarif::append_train_sarif::append_train_sarif;
use crate::sarif::log_sarif_json::log_sarif_json;

pub fn track_sarif(mut train: Train) -> Train {
    train = configure_sarif(train);

    train = set_sarif_settings(train);

    train = reset_train_warnings(train);

    train = append_train_sarif(train);

    train = log_sarif_json(train);

    train
}
