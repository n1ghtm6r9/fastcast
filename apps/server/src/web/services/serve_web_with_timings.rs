use crate::hub::Hub;
use crate::web::{ViewerTimings, WebError, router_with_timings};
use axum::serve::ListenerExt;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn serve_web_with_timings<S>(
    listener: TcpListener,
    hub: Arc<Hub>,
    timings: ViewerTimings,
    shutdown: S,
) -> Result<(), WebError>
where
    S: Future<Output = ()> + Send + 'static,
{
    let listener = listener.tap_io(|stream| {
        let _ = stream.set_nodelay(true);
    });

    axum::serve(listener, router_with_timings(hub, timings))
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(WebError::Serve)
}
