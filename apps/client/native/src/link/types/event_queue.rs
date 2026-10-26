use crate::link::QueueState;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct EventQueue {
    pub(crate) state: Mutex<QueueState>,
    pub(crate) ready: Condvar,
}
