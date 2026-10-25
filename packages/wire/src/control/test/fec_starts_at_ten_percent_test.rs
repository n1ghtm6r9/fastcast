use crate::control::FecController;
use std::time::Instant;

#[test]
fn fec_starts_at_ten_percent() {
    assert_eq!(FecController::new(Instant::now()).percent(), 10);
}
