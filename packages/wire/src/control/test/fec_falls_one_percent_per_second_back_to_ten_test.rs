use crate::control::FecController;
use std::time::{Duration, Instant};

#[test]
fn fec_falls_one_percent_per_second_back_to_ten() {
    let t0 = Instant::now();
    let mut controller = FecController::new(t0);

    controller.observe(0.05, t0);
    assert_eq!(controller.percent(), 15);

    controller.observe(0.0, t0 + Duration::from_millis(999));
    assert_eq!(controller.percent(), 15);

    controller.observe(0.0, t0 + Duration::from_secs(1));
    assert_eq!(controller.percent(), 14);

    controller.observe(0.0, t0 + Duration::from_millis(1500));
    assert_eq!(controller.percent(), 14);

    controller.observe(0.0, t0 + Duration::from_secs(2));
    assert_eq!(controller.percent(), 13);

    controller.observe(0.0, t0 + Duration::from_secs(10));
    assert_eq!(controller.percent(), 10);
}
