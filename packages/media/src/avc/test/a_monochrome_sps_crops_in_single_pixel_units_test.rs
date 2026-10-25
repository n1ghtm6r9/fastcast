use super::{SpsParts, video_size, video_size_of_parts};

#[test]
fn a_monochrome_sps_crops_in_single_pixel_units() {
    let parts = SpsParts {
        chroma_format_idc: 0,
        crop: Some([0, 10, 0, 0]),
        width_in_macroblocks: 86,
        height_in_map_units: 48,
        ..SpsParts::default()
    };

    assert_eq!(video_size_of_parts(&parts), Some(video_size(1366, 768)));
}
