use crate::hub::{Frame, StreamConfig};
use crate::web::{ViewerCursor, ViewerSink, send_admitted, send_config};
use std::sync::Arc;
use tokio::sync::broadcast::{self, error::RecvError};
use tracing::debug;

pub async fn write_to_viewer(
    sink: &ViewerSink,
    cursor: &mut ViewerCursor,
    config: Option<Arc<StreamConfig>>,
    gop: Vec<Arc<Frame>>,
    frames: &mut broadcast::Receiver<Arc<Frame>>,
) -> Result<(), axum::Error> {
    if let Some(config) = config {
        send_config(sink, &config).await?;
    }

    for frame in gop {
        send_admitted(sink, cursor, &frame).await?;
    }

    loop {
        match frames.recv().await {
            Ok(frame) => send_admitted(sink, cursor, &frame).await?,
            Err(RecvError::Lagged(skipped)) => {
                debug!("a viewer fell {skipped} frames behind, waiting for the next key frame");
                cursor.lagged();
            }
            Err(RecvError::Closed) => return Ok(()),
        }
    }
}
