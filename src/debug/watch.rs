use crate::state::{Train};

/// Logs the current watch message and level from the Train struct.
pub fn watch(mut train: Train) -> Train {
    // Ensure the message is not empty before logging
    if !train.watch.message.is_empty() {
        eprintln!("WATCH [Level {}]: {}", train.watch.level, train.watch.message);
    }

    train
}
