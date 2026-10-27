use crate::hub::Hub;

#[test]
fn no_diagnosis_is_asked_before_the_debug_box_is_ticked() {
    assert_eq!(Hub::new().join().diagnose, None);
}
