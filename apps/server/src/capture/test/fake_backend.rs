use super::FakeRun;
use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

pub struct FakeBackend {
    pub(super) script: Mutex<VecDeque<FakeRun>>,
    pub(super) started: Arc<AtomicUsize>,
    pub(super) killed: Arc<AtomicUsize>,
}

impl FakeBackend {
    pub fn scripted(script: Vec<FakeRun>) -> Arc<Self> {
        Arc::new(Self {
            script: Mutex::new(script.into()),
            started: Arc::new(AtomicUsize::new(0)),
            killed: Arc::new(AtomicUsize::new(0)),
        })
    }
}
