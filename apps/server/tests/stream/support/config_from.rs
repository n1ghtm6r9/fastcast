use app::hub::{StreamConfig, stream_config_from_record};
use bytes::Bytes;
use media::VideoCodec;

pub fn config_from(avcc: &[u8], fps: u32) -> StreamConfig {
    stream_config_from_record(VideoCodec::Avc, Bytes::copy_from_slice(avcc), fps)
        .expect("the avcC describes a stream")
}
