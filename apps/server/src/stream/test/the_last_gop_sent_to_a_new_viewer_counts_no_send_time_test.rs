use super::{hub_with_x264_config, next_frame, next_welcome, start_session};
use crate::hub::ViewerRequest;
use std::time::{Duration, Instant};
use wire::FrameReceiver;

#[test]
fn the_last_gop_sent_to_a_new_viewer_counts_no_send_time() {
    let hub = hub_with_x264_config();
    let shown = Instant::now() - Duration::from_secs(1);

    hub.publish_shown(true, 0, &[1; 3000], Some(shown));
    hub.publish_shown(false, 16, &[2; 200], Some(shown));

    let session = start_session(&hub, ViewerRequest::default());
    let control = session.control.clone();
    let mut receiver = FrameReceiver::new();

    next_welcome(&session.peer);

    for _ in 0..3 {
        next_frame(&session.peer, &mut receiver);
    }

    session.stop_and_join();

    assert!(control.take_sent().is_empty());
}
