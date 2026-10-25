use crate::hub::{Hub, Subscription, ViewerRequest};
use crate::web::{
    ViewerCursor, ViewerSink, ViewerTimings, ping_viewer, watch_for_close, write_to_viewer,
};
use axum::extract::ws::WebSocket;
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::debug;

pub async fn serve_viewer(
    socket: WebSocket,
    hub: Arc<Hub>,
    request: ViewerRequest,
    timings: ViewerTimings,
) {
    debug!("a viewer asks for {request:?}");
    hub.request(request);

    let Subscription {
        config,
        gop,
        mut frames,
        guard: viewer,
    } = hub.join();
    let (sink, stream) = socket.split();
    let sink = ViewerSink::new(sink, timings.write_timeout);
    let mut cursor = ViewerCursor::starting_with(config.clone());

    debug!("a viewer connected");

    tokio::select! {
        () = watch_for_close(stream, timings.silence_timeout) => {}
        error = ping_viewer(&sink, timings.ping_interval) => {
            debug!("pinging a viewer failed: {error}");
        }
        result = write_to_viewer(&sink, &mut cursor, config, gop, &mut frames) => {
            if let Err(error) = result {
                debug!("writing to a viewer failed: {error}");
            }
        }
    }

    drop(viewer);
    sink.close().await;

    debug!("a viewer left");
}
