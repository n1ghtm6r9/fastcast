use crate::support::RunningServer;
use app::hub::Hub;
use app::web::{ViewerTimings, bind_listener, serve_web_with_timings};
use std::sync::Arc;
use tokio::sync::oneshot;

pub async fn start_server_with_timings(hub: Arc<Hub>, timings: ViewerTimings) -> RunningServer {
    let listener = bind_listener("127.0.0.1", 0)
        .await
        .expect("a free port exists");
    let address = listener
        .local_addr()
        .expect("a bound listener has an address");
    let (shutdown, signal) = oneshot::channel::<()>();
    let task = tokio::spawn(serve_web_with_timings(listener, hub, timings, async move {
        let _ = signal.await;
    }));

    RunningServer {
        address,
        shutdown,
        task,
    }
}
