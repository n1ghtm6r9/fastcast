use crate::hub::{Hub, Subscription};
use crate::web::{VIEWER_CLOSE_TIMEOUT, ViewerCursor, watch_for_close, write_to_viewer};
use axum::extract::ws::WebSocket;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::time::timeout;
use tracing::debug;

pub async fn serve_viewer(socket: WebSocket, hub: Arc<Hub>) {
    let Subscription {
        config,
        gop,
        mut frames,
        guard: _viewer,
    } = hub.join();
    let (mut sink, stream) = socket.split();
    let mut cursor = ViewerCursor::starting_with(config.clone());

    debug!("a viewer connected");

    tokio::select! {
        () = watch_for_close(stream) => {}
        result = write_to_viewer(&mut sink, &mut cursor, config, gop, &mut frames) => {
            if let Err(error) = result {
                debug!("writing to a viewer failed: {error}");
            }
        }
    }

    let _ = timeout(VIEWER_CLOSE_TIMEOUT, sink.close()).await;

    debug!("a viewer left");
}
