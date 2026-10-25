use super::hold_gpu_slot;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{scope, sleep};
use std::time::Duration;

#[test]
fn gpu_tests_never_hold_more_than_four_encoder_slots() {
    let holding = AtomicUsize::new(0);
    let most_holding = AtomicUsize::new(0);

    scope(|threads| {
        for _ in 0..12 {
            threads.spawn(|| {
                hold_gpu_slot();
                let now = holding.fetch_add(1, Ordering::SeqCst) + 1;
                most_holding.fetch_max(now, Ordering::SeqCst);
                sleep(Duration::from_millis(30));
                holding.fetch_sub(1, Ordering::SeqCst);
            });
        }
    });

    assert!(most_holding.load(Ordering::SeqCst) <= 4);
}
