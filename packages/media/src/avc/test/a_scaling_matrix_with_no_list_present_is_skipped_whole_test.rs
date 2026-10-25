use super::{ScalingListParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn a_scaling_matrix_with_no_list_present_is_skipped_whole() {
    let parts = SpsParts {
        scaling_matrix: Some(vec![ScalingListParts::Absent; 8]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
