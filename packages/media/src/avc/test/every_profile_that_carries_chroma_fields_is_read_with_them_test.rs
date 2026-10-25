use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn every_profile_that_carries_chroma_fields_is_read_with_them() {
    for profile_idc in [100, 110, 122, 244, 44, 83, 86, 118, 128, 138, 139, 134, 135] {
        let parts = SpsParts {
            profile_idc,
            ..SpsParts::default()
        };

        assert_eq!(
            video_size_of_parts(&parts),
            Some(video_size(1920, 1080)),
            "profile {profile_idc}"
        );
    }
}
