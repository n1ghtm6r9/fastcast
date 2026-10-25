use super::{avc_tag_data, demux_all, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn a_timestamp_past_24_bits_takes_its_high_byte_from_the_extended_field() {
    let below_the_extension = 0x00ff_ffff;
    let past_the_extension = 0x0100_0010;
    let the_largest = u32::MAX;
    let frame = [0, 0, 0, 1, 0x41];
    let stream = [
        flv_header_bytes(),
        flv_tag_bytes(9, below_the_extension, &avc_tag_data(2, 1, &frame)),
        flv_tag_bytes(9, past_the_extension, &avc_tag_data(2, 1, &frame)),
        flv_tag_bytes(9, the_largest, &avc_tag_data(2, 1, &frame)),
    ]
    .concat();

    let timestamps: Vec<u32> = demux_all(stream)
        .await
        .into_iter()
        .map(|event| match event {
            FlvEvent::Frame {
                timestamp_ms, data, ..
            } => {
                assert_eq!(data, Bytes::copy_from_slice(&frame));
                timestamp_ms
            }
            other => panic!("only frames were sent, got {:?}", other),
        })
        .collect();

    assert_eq!(
        timestamps,
        vec![below_the_extension, past_the_extension, the_largest]
    );
}
