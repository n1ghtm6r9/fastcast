use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn duplicates_never_make_loss_negative() {
    let mut meter = LossMeter::new();

    meter.update(&feedback(100, 99));

    assert_eq!(meter.update(&feedback(220, 199)), Some(0.0));
}
