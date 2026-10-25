use crate::hub::Hub;
use crate::web::{ViewerTimings, WebError, serve_web_with_timings};
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn serve_web<S>(listener: TcpListener, hub: Arc<Hub>, shutdown: S) -> Result<(), WebError>
where
    S: Future<Output = ()> + Send + 'static,
{
    serve_web_with_timings(listener, hub, ViewerTimings::default(), shutdown).await
}
