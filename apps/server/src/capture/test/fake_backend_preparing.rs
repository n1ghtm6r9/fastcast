use super::{FakeBackend, FakePreparation, FakeRun};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

impl FakeBackend {
    pub fn preparing(preparations: Vec<FakePreparation>, script: Vec<FakeRun>) -> Arc<Self> {
        Arc::new(Self {
            preparations: Mutex::new(preparations.into()),
            script: Mutex::new(script.into()),
            prepared: Arc::new(AtomicUsize::new(0)),
            started: Arc::new(AtomicUsize::new(0)),
            killed: Arc::new(AtomicUsize::new(0)),
            opened: Mutex::new(Vec::new()),
        })
    }
}
