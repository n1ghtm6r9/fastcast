use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tracing::warn;

pub async fn forward_stderr<R: AsyncRead + Unpin>(stderr: R) {
    let mut lines = BufReader::new(stderr).split(b'\n');

    while let Ok(Some(segment)) = lines.next_segment().await {
        let text = String::from_utf8_lossy(&segment);
        let line = text.trim();

        if !line.is_empty() {
            warn!(target: "ffmpeg", "{line}");
        }
    }
}
