use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct FecController {
    pub(crate) percent: u8,
    pub(crate) last_step: Instant,
}
