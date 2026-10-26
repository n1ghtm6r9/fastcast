use crate::stream::clipped;

#[test]
fn a_long_report_is_clipped_on_a_character_boundary() {
    assert_eq!(clipped("кадры: 60", 5), "кадры");
}
