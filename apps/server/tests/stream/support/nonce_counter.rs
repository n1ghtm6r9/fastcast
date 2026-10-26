use std::sync::atomic::AtomicU32;

pub static NONCE_COUNTER: AtomicU32 = AtomicU32::new(1);
