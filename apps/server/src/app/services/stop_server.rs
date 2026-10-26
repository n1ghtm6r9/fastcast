use crate::app::RUNTIME_SHUTDOWN_LIMIT;
use crate::server::ServerControl;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::timeout;
use tracing::warn;

pub fn stop_server(runtime: Runtime, control: ServerControl, limit: Duration) {
    let stopped = runtime.block_on(async { timeout(limit, control.stop()).await });

    if stopped.is_err() {
        warn!(
            "the server did not stop within {} ms, fastcast closes anyway",
            limit.as_millis()
        );
    }
    runtime.shutdown_timeout(RUNTIME_SHUTDOWN_LIMIT);
}
