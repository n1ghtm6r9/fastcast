use super::frame_timestamp;
use crate::fixtures::hub_with_config;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn a_joiner_never_misses_or_repeats_a_frame_while_frames_keep_coming() {
    let hub = Arc::new(hub_with_config());
    let running = Arc::new(AtomicBool::new(true));

    let publishing = {
        let hub = hub.clone();
        let running = running.clone();

        thread::spawn(move || {
            let mut index: u32 = 0;

            while running.load(Ordering::Relaxed) {
                hub.publish(index.is_multiple_of(50), index, &[1]);
                index += 1;
            }
        })
    };

    let deadline = Instant::now() + Duration::from_secs(10);
    let mut checked = 0;

    while checked < 200 && Instant::now() < deadline {
        let mut subscription = hub.join();

        let Some(last_in_gop) = subscription.gop.last() else {
            continue;
        };

        let last_timestamp = frame_timestamp(last_in_gop);

        if let Ok(next) = subscription.frames.try_recv() {
            assert_eq!(frame_timestamp(&next), last_timestamp + 1);
            checked += 1;
        }
    }

    running.store(false, Ordering::Relaxed);
    publishing.join().expect("the publisher stops");

    assert_eq!(checked, 200);
}
