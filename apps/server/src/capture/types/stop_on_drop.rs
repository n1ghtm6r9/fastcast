use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub struct StopOnDrop(pub Arc<AtomicBool>);
