use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct LossDeadline {
    pub at: Instant,
    pub end: u64,
}
