use super::{FakePreparation, FakeRun};
use crate::hub::ViewerRequest;
use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

pub struct FakeBackend {
    pub(super) preparations: Mutex<VecDeque<FakePreparation>>,
    pub(super) script: Mutex<VecDeque<FakeRun>>,
    pub(super) prepared: Arc<AtomicUsize>,
    pub(super) started: Arc<AtomicUsize>,
    pub(super) killed: Arc<AtomicUsize>,
    pub(super) opened: Mutex<Vec<ViewerRequest>>,
}

impl FakeBackend {
    pub fn scripted(script: Vec<FakeRun>) -> Arc<Self> {
        Self::preparing(Vec::new(), script)
    }
}
