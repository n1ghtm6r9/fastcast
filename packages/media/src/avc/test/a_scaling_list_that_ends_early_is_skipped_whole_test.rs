use super::{ScalingListParts, SpsParts, video_size, video_size_of_parts};

#[test]
fn a_scaling_list_that_ends_early_is_skipped_whole() {
    let parts = SpsParts {
        scaling_matrix: Some(vec![
            ScalingListParts::EndsAfter(0),
            ScalingListParts::EndsAfter(5),
            ScalingListParts::Absent,
            ScalingListParts::Full,
            ScalingListParts::Absent,
            ScalingListParts::Absent,
            ScalingListParts::EndsAfter(20),
            ScalingListParts::Full,
        ]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
