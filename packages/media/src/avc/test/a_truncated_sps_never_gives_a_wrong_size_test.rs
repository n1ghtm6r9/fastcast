use super::{SpsParts, video_size};
use crate::avc::video_size_from_sps;

#[test]
fn a_truncated_sps_never_gives_a_wrong_size() {
    let nal_unit = SpsParts::default().nal_unit();
    let expected = video_size(1920, 1080);

    for length in 0..=nal_unit.len() {
        let size = video_size_from_sps(&nal_unit[..length]);

        assert!(
            size.is_none() || size == Some(expected),
            "{length} bytes gave {size:?}"
        );

        if length <= 6 {
            assert_eq!(size, None, "{length} bytes");
        }
    }
}
