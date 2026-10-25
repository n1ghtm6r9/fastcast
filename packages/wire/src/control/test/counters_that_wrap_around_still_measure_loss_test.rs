use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn counters_that_wrap_around_still_measure_loss() {
    let mut meter = LossMeter::new();

    meter.update(&feedback(u32::MAX - 5, u32::MAX - 5));
    let loss = meter.update(&feedback(3, 4)).expect("a loss figure");

    assert!((loss - 0.1).abs() < 1e-9);
}
