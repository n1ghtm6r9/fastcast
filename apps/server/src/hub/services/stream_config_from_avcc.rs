use crate::hub::{CONFIG_MESSAGE_KIND, ConfigMessage, StreamConfig, config_message_json};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bytes::Bytes;
use media::{avc_codec_string, video_size_from_avcc};

pub fn stream_config_from_avcc(avcc: Bytes, fps: u32) -> Option<StreamConfig> {
    let codec = avc_codec_string(&avcc)?;
    let size = video_size_from_avcc(&avcc)?;

    let json = config_message_json(&ConfigMessage {
        kind: CONFIG_MESSAGE_KIND,
        codec: &codec,
        width: size.width,
        height: size.height,
        fps,
        description: &STANDARD.encode(&avcc),
    });

    Some(StreamConfig {
        avcc,
        codec,
        width: size.width,
        height: size.height,
        fps,
        json,
    })
}
