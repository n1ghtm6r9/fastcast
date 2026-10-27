use crate::stream::scale_step_of;

#[test]
fn a_scale_notice_carries_its_step() {
    assert_eq!(scale_step_of(r#"{"type":"scale","step":-1}"#), Some(-1));
}
