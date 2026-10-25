use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn the_separate_colour_plane_flag_is_read_for_4_4_4_streams() {
    let parts = SpsParts {
        profile_idc: 244,
        chroma_format_idc: 3,
        separate_colour_plane: true,
        crop: Some([0, 0, 0, 8]),
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1920, 1080)));
}
