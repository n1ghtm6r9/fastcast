use app::hub::{StreamConfig, stream_config_from_avcc};
use bytes::Bytes;

pub fn config_from(avcc: &[u8], fps: u32) -> StreamConfig {
    stream_config_from_avcc(Bytes::copy_from_slice(avcc), fps).expect("the avcC describes a stream")
}
