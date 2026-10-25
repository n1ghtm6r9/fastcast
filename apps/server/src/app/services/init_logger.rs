use std::io::{IsTerminal, stdout};
use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(stdout().is_terminal())
        .try_init();
}
