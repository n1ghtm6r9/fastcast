use std::sync::atomic::{AtomicU64, Ordering};

pub fn next_shape_generation() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);

    NEXT.fetch_add(1, Ordering::Relaxed)
}
