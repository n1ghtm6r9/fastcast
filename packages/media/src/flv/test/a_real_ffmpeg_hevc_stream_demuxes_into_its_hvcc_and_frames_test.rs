use super::{demux_all, nal_unit_types_of, real_enhanced_flv_stream_named};
use crate::avc::test::video_size;
use crate::flv::{FlvEvent, VideoCodec};
use crate::{hevc_codec_string, video_size_from_hvcc};

#[tokio::test]
async fn a_real_ffmpeg_hevc_stream_demuxes_into_its_hvcc_and_frames() {
    let stream = real_enhanced_flv_stream_named("hevc_nvenc_192x108_without_b_frames");
    let events = demux_all(stream).await;

    let FlvEvent::SequenceHeader { codec, record } = &events[0] else {
        panic!(
            "the stream starts with a sequence header, not {:?}",
            events[0]
        );
    };

    assert_eq!(*codec, VideoCodec::Hevc);
    assert_eq!(
        hevc_codec_string(record),
        Some("hvc1.1.6.L60.90".to_string())
    );
    assert_eq!(video_size_from_hvcc(record), Some(video_size(192, 108)));

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
            (false, 17, vec![39, 1]),
            (false, 33, vec![39, 1]),
            (true, 50, vec![39, 19]),
        ]
    );
}
