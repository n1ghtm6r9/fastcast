use super::{ScalingListParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn a_4_4_4_stream_has_twelve_scaling_lists() {
    let parts = SpsParts {
        profile_idc: 244,
        chroma_format_idc: 3,
        scaling_matrix: Some(vec![ScalingListParts::Full; 12]),
        crop: Some([0, 0, 0, 8]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
