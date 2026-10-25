use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn a_4_4_4_stream_with_separate_colour_planes_is_read_through_its_extra_flag() {
    let parts = HevcSpsParts {
        chroma_format_idc: 3,
        separate_colour_plane: true,
        conformance_window: Some([0, 0, 0, 8]),
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1920, 1080))
    );
}
