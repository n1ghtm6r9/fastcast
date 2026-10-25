use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn each_feedback_is_compared_with_the_one_before() {
    let mut meter = LossMeter::new();

    meter.update(&feedback(100, 99));
    meter.update(&feedback(190, 199));

    assert_eq!(meter.update(&feedback(290, 299)), Some(0.0));
}
