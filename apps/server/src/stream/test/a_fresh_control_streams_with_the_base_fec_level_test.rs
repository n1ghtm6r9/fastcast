use crate::stream::SessionControl;

#[test]
fn a_fresh_control_streams_with_the_base_fec_level() {
    assert_eq!(SessionControl::new().fec_percent(), 10);
}
