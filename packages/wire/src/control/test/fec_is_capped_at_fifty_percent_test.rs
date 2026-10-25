use crate::control::FecController;
use std::time::Instant;

#[test]
fn fec_is_capped_at_fifty_percent() {
    let t0 = Instant::now();
    let mut controller = FecController::new(t0);

    controller.observe(0.4, t0);

    assert_eq!(controller.percent(), 50);
}
