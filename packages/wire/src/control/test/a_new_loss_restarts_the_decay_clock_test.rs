use crate::control::FecController;
use std::time::{Duration, Instant};

#[test]
fn a_new_loss_restarts_the_decay_clock() {
    let t0 = Instant::now();
    let mut controller = FecController::new(t0);

    controller.observe(0.05, t0);
    controller.observe(0.1, t0 + Duration::from_millis(900));
    assert_eq!(controller.percent(), 30);

    controller.observe(0.0, t0 + Duration::from_millis(1800));
    assert_eq!(controller.percent(), 30);

    controller.observe(0.0, t0 + Duration::from_millis(1900));
    assert_eq!(controller.percent(), 29);
}
