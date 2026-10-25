use crate::control::FecController;
use std::time::Instant;

#[test]
fn fec_jumps_to_three_times_the_loss() {
    let t0 = Instant::now();
    let mut controller = FecController::new(t0);

    controller.observe(0.05, t0);

    assert_eq!(controller.percent(), 15);
}
