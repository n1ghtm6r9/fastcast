use super::{Av1SequenceHeaderParts, av1c_of_parts};

#[test]
fn frame_ids_order_hints_and_forced_screen_content_tools_are_read_past() {
    let parts = Av1SequenceHeaderParts {
        frame_id_numbers: true,
        order_hint: true,
        choose_screen_content_tools: false,
        force_screen_content_tools: true,
        choose_integer_mv: false,
        high_bitdepth: true,
        chroma_sample_position: 2,
        ..Av1SequenceHeaderParts::default()
    };

    let av1c = av1c_of_parts(&parts).expect("the header is read");

    assert_eq!(av1c[..4], [0x81, 0x08, 0x4E, 0x00]);
}
