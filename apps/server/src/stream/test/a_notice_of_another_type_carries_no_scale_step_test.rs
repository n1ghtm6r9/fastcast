use crate::stream::scale_step_of;

#[test]
fn a_notice_of_another_type_carries_no_scale_step() {
    assert_eq!(scale_step_of(r#"{"type":"zoom","step":1}"#), None);
}
