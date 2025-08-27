use crate::state::Train;

pub fn watch(train: Train) -> Train { 
    eprintln!("WATCH [Level: {}]: {}", train.watch.level, train.watch.message);

    train
}
