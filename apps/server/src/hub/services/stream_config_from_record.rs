use crate::hub::{
    CONFIG_MESSAGE_KIND, ConfigMessage, StreamConfig, codec_string_and_size, config_message_json,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bytes::Bytes;
use media::VideoCodec;

pub fn stream_config_from_record(
    codec: VideoCodec,
    record: Bytes,
    fps: u32,
) -> Option<StreamConfig> {
    let (name, size) = codec_string_and_size(codec, &record)?;

    let json = config_message_json(&ConfigMessage {
        kind: CONFIG_MESSAGE_KIND,
        codec: &name,
        width: size.width,
        height: size.height,
        fps,
        description: &STANDARD.encode(&record),
    });

    Some(StreamConfig {
        record,
        codec: name,
        width: size.width,
        height: size.height,
        fps,
        json,
    })
}
