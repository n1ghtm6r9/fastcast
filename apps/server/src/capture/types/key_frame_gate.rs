use std::time::Instant;

#[derive(Default)]
pub struct KeyFrameGate {
    pub(crate) last: Option<Instant>,
}
