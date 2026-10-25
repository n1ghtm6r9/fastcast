use super::{HevcSpsParts, video_size_of_hevc_parts};
use crate::avc::test::video_size;

#[test]
fn a_monochrome_sps_crops_in_single_pixel_units() {
    let parts = HevcSpsParts {
        chroma_format_idc: 0,
        width_in_luma_samples: 1376,
        height_in_luma_samples: 768,
        conformance_window: Some([0, 10, 0, 0]),
        ..HevcSpsParts::default()
    };

    assert_eq!(
        video_size_of_hevc_parts(&parts),
        Some(video_size(1366, 768))
    );
}
