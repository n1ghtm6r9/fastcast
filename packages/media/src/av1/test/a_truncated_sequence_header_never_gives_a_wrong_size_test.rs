use super::{config_obus_of, obu_of, real_av1c_record_named};
use crate::avc::test::video_size;
use crate::video_size_from_av1c;

#[test]
fn a_truncated_sequence_header_never_gives_a_wrong_size() {
    let av1c = real_av1c_record_named("nvenc_main10_3840x2160_120").av1c;
    let payload = &config_obus_of(&av1c)[2..];
    let expected = video_size(3840, 2160);

    for length in 0..=payload.len() {
        let truncated = [&av1c[..4], &obu_of(1, &payload[..length])].concat();
        let size = video_size_from_av1c(&truncated);

        assert!(
            size.is_none() || size == Some(expected),
            "{length} bytes gave {size:?}"
        );

        if length < 8 {
            assert_eq!(size, None, "{length} bytes");
        }
    }

    assert_eq!(video_size_from_av1c(&av1c), Some(expected));
}
