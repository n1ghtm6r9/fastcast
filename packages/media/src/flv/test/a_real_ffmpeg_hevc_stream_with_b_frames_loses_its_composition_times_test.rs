use super::{demux_all, nal_unit_types_of, real_enhanced_flv_stream_named};
use crate::flv::{FlvEvent, VideoCodec};

#[tokio::test]
async fn a_real_ffmpeg_hevc_stream_with_b_frames_loses_its_composition_times() {
    let stream = real_enhanced_flv_stream_named("hevc_nvenc_192x108_with_b_frames");
    let events = demux_all(stream).await;

    assert!(matches!(
        events[0],
        FlvEvent::SequenceHeader {
            codec: VideoCodec::Hevc,
            ..
        }
    ));

    let frames: Vec<(bool, u32, Vec<u8>)> = events[1..]
        .iter()
        .map(|event| match event {
            FlvEvent::Frame {
                key,
                timestamp_ms,
                data,
            } => (*key, *timestamp_ms, nal_unit_types_of(data)),
            other => panic!("a frame was expected, not {other:?}"),
        })
        .collect();

    assert_eq!(
        frames,
        vec![
            (true, 0, vec![39, 19]),
            (false, 16, vec![39, 1]),
            (false, 33, vec![39, 1]),
            (false, 50, vec![39, 0]),
            (false, 66, vec![39, 1]),
            (false, 83, vec![39, 1]),
        ]
    );
}
