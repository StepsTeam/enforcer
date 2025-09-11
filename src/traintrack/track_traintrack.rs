use crate::state::Train;

use crate::sarif::set_sarif_settings;
use crate::traintrack::configure_traintrack::configure_traintrack;
use crate::traintrack::check_one_function_per_file::check_one_function_per_file;
use crate::traintrack::check_train_function_input::check_train_function_input;
use crate::traintrack::check_train_function_output::check_train_function_output;
use crate::traintrack::check_train_size_limit::check_train_size_limit;
use crate::traintrack::check_function_nesting_limit::check_function_nesting_limit;

pub fn track_traintrack(mut train: Train) -> Train {
    train = configure_traintrack(train);

    train = set_sarif_settings(train);

    train = check_one_function_per_file(train);

    train = check_train_function_input(train);

    train = check_train_size_limit(train);

    train = check_train_function_output(train);

    train = check_function_nesting_limit(train);

    train
}
