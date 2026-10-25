use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn a_hand_built_sps_gives_the_size_it_was_built_with() {
    assert_eq!(
        video_size_of_parts(&SpsParts::default()),
        Some(video_size(1920, 1080))
    );
}
