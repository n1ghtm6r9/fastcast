use crate::hub::Hub;
use crate::web::{WebError, router};
use axum::serve::ListenerExt;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn serve_web<S>(listener: TcpListener, hub: Arc<Hub>, shutdown: S) -> Result<(), WebError>
where
    S: Future<Output = ()> + Send + 'static,
{
    let listener = listener.tap_io(|stream| {
        let _ = stream.set_nodelay(true);
    });

    axum::serve(listener, router(hub))
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(WebError::Serve)
}
