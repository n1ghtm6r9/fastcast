use crate::control::LossMeter;
use crate::control::test::feedback;

#[test]
fn without_new_sequences_there_is_no_loss() {
    let mut meter = LossMeter::new();

    meter.update(&feedback(100, 99));

    assert_eq!(meter.update(&feedback(100, 99)), None);
    assert_eq!(meter.update(&feedback(130, 129)), Some(0.0));
}
