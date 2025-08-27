use crate::state::Train;
use crate::oewl::acquire_oewl::acquire_oewl;
use crate::oewl::configure_oewl::configure_oewl;

pub fn track_oewl(mut train: Train) -> Train {
    // First, ensure OEWL data is acquired 
    // build.rs then confirmed by acquire_oewl
    train = acquire_oewl(train); 

    train = configure_oewl(train);
    
    train
}
