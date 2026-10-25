use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn profiles_without_chroma_fields_are_read_as_4_2_0() {
    for profile_idc in [66, 77, 88] {
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
