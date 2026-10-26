use crate::support::NONCE_COUNTER;
use std::sync::atomic::Ordering;

pub fn next_nonce() -> u32 {
    NONCE_COUNTER.fetch_add(1, Ordering::Relaxed)
}
