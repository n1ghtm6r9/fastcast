use std::panic;
use tracing::error;

pub fn log_panics() {
    let previous = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        error!("fastcast {}", info.to_string().replace('\n', " "));
        previous(info);
    }));
}
