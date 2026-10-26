use super::{Av1SequenceHeaderParts, av1c_of_parts};

#[test]
fn screen_content_tools_switched_off_have_no_integer_mv_flags() {
    let parts = Av1SequenceHeaderParts {
        choose_screen_content_tools: false,
        force_screen_content_tools: false,
        high_bitdepth: true,
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1c[..4], [0x81, 0x08, 0x4D, 0x00]);
}
