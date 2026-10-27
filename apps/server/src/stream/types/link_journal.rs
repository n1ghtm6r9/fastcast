use std::time::Instant;
use wire::Feedback;

#[derive(Debug, Clone, Copy, Default)]
pub struct LinkJournal {
    pub(crate) since: Option<(Instant, Feedback)>,
    pub(crate) keyframe_asks: u32,
    pub(crate) asking: bool,
}
