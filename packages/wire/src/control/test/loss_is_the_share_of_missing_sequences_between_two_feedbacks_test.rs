use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn loss_is_the_share_of_missing_sequences_between_two_feedbacks() {
    let mut meter = LossMeter::new();

    meter.update(&feedback(100, 99));
    let loss = meter.update(&feedback(190, 199)).expect("a loss figure");

    assert!((loss - 0.1).abs() < 1e-9);
}
