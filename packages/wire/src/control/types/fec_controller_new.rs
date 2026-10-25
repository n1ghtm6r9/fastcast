use crate::control::{BASE_FEC_PERCENT, FecController};
use std::time::Instant;

impl FecController {
    pub fn new(now: Instant) -> FecController {
        FecController {
            percent: BASE_FEC_PERCENT,
            last_step: now,
        }
    }
}
