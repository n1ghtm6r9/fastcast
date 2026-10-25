use super::{demux_all, enhanced_tag_data, flv_header_bytes, flv_tag_bytes};
use crate::flv::FlvEvent;
use bytes::Bytes;

#[tokio::test]
async fn enhanced_packets_other_than_sequence_start_and_coded_frames_are_skipped() {
    let key_frame = [0, 0, 0, 2, 0x26, 7];
    let skipped_packets = [(1, 2), (5, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 15)];
    let mut stream = flv_header_bytes();

    for (frame_type, packet_type) in skipped_packets {
        let data = enhanced_tag_data(frame_type, packet_type, b"hvc1", &[1, 2, 3, 4]);

        stream.extend(flv_tag_bytes(9, 0, &data));
    }

    stream.extend(flv_tag_bytes(
        9,
        8,
        &enhanced_tag_data(1, 3, b"hvc1", &key_frame),
    ));

    assert_eq!(
        demux_all(stream).await,
        vec![FlvEvent::Frame {
            key: true,
            timestamp_ms: 8,
            data: Bytes::copy_from_slice(&key_frame),
        }]
    );
}
