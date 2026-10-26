use crate::link::LinkEvent;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct QueueState {
    pub(crate) events: VecDeque<LinkEvent>,
    pub(crate) frames: usize,
    pub(crate) waiting_for_key_frame: bool,
    pub(crate) closed: bool,
}
