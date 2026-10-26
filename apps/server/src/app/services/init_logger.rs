use std::io::{IsTerminal, stdout};
use tracing::Level;

pub fn init_logger() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(stdout().is_terminal())
        .try_init();
}
