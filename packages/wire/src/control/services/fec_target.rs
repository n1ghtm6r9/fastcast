use crate::control::{FEC_LOSS_FACTOR, MAX_FEC_PERCENT};

pub fn fec_target(loss: f64) -> u8 {
    (loss * 100.0 * f64::from(FEC_LOSS_FACTOR))
        .ceil()
        .min(f64::from(MAX_FEC_PERCENT)) as u8
}
