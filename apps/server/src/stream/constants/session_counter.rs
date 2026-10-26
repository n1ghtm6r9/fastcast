use std::sync::atomic::AtomicU32;

pub static SESSION_COUNTER: AtomicU32 = AtomicU32::new(0);
