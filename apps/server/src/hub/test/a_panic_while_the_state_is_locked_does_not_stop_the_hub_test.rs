use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::Hub;
use std::sync::Arc;
use std::thread;

#[test]
fn a_panic_while_the_state_is_locked_does_not_stop_the_hub() {
    let hub = Arc::new(Hub::new());
    let poisoner = hub.clone();

    let outcome = thread::spawn(move || {
        let _state = poisoner.lock_state();

        panic!("a thread died holding the hub state");
    })
    .join();

    assert!(outcome.is_err());
    assert!(hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60)));
    assert!(hub.join().config.is_some());
}
