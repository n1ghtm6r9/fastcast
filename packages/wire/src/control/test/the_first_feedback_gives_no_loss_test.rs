use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn the_first_feedback_gives_no_loss() {
    let mut meter = LossMeter::new();

    assert_eq!(meter.update(&feedback(100, 99)), None);
}
