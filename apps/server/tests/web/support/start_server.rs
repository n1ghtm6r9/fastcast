use crate::support::RunningServer;
use app::hub::Hub;
use app::web::{bind_listener, serve_web};
use std::sync::Arc;
use tokio::sync::oneshot;

pub async fn start_server(hub: Arc<Hub>) -> RunningServer {
    let listener = bind_listener("127.0.0.1", 0)
        .await
        .expect("a free port exists");
    let address = listener
        .local_addr()
        .expect("a bound listener has an address");
    let (shutdown, signal) = oneshot::channel::<()>();
    let task = tokio::spawn(serve_web(listener, hub, async move {
        let _ = signal.await;
    }));

    RunningServer {
        address,
        shutdown,
        task,
    }
}
