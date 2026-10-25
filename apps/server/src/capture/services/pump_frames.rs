use crate::capture::{CaptureError, CaptureReport};
use crate::hub::{Hub, stream_config_from_avcc};
use media::{FlvDemuxer, FlvEvent};
use tokio::io::AsyncRead;
use tracing::{info, warn};

pub async fn pump_frames<R: AsyncRead + Unpin>(
    reader: R,
    hub: &Hub,
    fps: u32,
) -> Result<CaptureReport, CaptureError> {
    let mut demuxer = FlvDemuxer::new(reader);
    let mut frames = 0;

    while let Some(event) = demuxer.next_event().await? {
        match event {
            FlvEvent::SequenceHeader(avcc) => match stream_config_from_avcc(avcc, fps) {
                Some(config) => {
                    info!(
                        "stream is {} {}x{} at {} fps",
                        config.codec, config.width, config.height, config.fps
                    );
                    hub.set_config(config);
                }
                None => warn!("ffmpeg sent a sequence header the server cannot describe"),
            },
            FlvEvent::Frame {
                key,
                timestamp_ms,
                data,
            } => {
                hub.publish(key, timestamp_ms, &data);
                frames += 1;
            }
        }
    }

    Ok(CaptureReport { frames })
}
