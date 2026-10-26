use std::sync::atomic::{AtomicI64, Ordering};

pub fn next_link_id() -> i64 {
    static NEXT: AtomicI64 = AtomicI64::new(1);

    NEXT.fetch_add(1, Ordering::Relaxed)
}
