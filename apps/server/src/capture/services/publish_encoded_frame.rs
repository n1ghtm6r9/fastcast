use crate::capture::NativeError;
use crate::hub::{Hub, stream_config_from_record};
use media::{VideoCodec, split_access_unit};
use std::time::Instant;

pub fn publish_encoded_frame(
    hub: &Hub,
    codec: VideoCodec,
    fps: u32,
    keyframe: bool,
    annex_b: &[u8],
    timestamp_ms: u32,
    shown: Option<Instant>,
) -> Result<bool, NativeError> {
    if annex_b.is_empty() {
        return Ok(false);
    }

    let access_unit = split_access_unit(codec, annex_b);

    let config = access_unit
        .record
        .and_then(|record| stream_config_from_record(codec, record.into(), fps));

    match config {
        Some(config) => {
            hub.set_config(config);
        }
        None if keyframe => return Err(NativeError::UndescribableStream),
        None => {}
    }

    hub.publish_shown(keyframe, timestamp_ms, &access_unit.payload, shown);

    Ok(true)
}
