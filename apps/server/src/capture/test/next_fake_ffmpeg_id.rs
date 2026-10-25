use std::sync::atomic::{AtomicUsize, Ordering};

pub fn next_fake_ffmpeg_id() -> usize {
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    NEXT.fetch_add(1, Ordering::SeqCst)
}
