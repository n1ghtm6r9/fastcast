use crate::avc::test::hex_bytes;

const STREAMS: &str = include_str!("real_enhanced_flv_streams.txt");

pub fn real_enhanced_flv_stream_named(name: &str) -> Vec<u8> {
    STREAMS
        .lines()
        .filter_map(|line| line.split_once(' '))
        .find(|(stream_name, _)| *stream_name == name)
        .map(|(_, hex)| hex_bytes(hex.trim()))
        .expect("the stream exists")
}
