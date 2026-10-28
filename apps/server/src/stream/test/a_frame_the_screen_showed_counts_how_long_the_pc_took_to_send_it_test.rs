use super::{bind_peer, frame_under, loopback_stream_socket};
use crate::fixtures::{config_of, x264_1920x1080_avcc};
use crate::hub::Frame;
use crate::stream::{SessionControl, SessionSender};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn a_frame_the_screen_showed_counts_how_long_the_pc_took_to_send_it() {
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let control = Arc::new(SessionControl::new());
    let socket = Arc::new(loopback_stream_socket());
    let mut sender = SessionSender::new(socket, address, 1, control.clone());
    let config = Arc::new(config_of(&x264_1920x1080_avcc(), 60));
    let shown = Instant::now() - Duration::from_millis(5);
    let frame = Frame {
        shown: Some(shown),
        ..frame_under(&config, true)
    };

    sender.send_video(&frame);

    let sent = control.take_sent();
    assert_eq!(sent.len(), 1);
    assert!(sent[0] >= Duration::from_millis(5), "{:?}", sent[0]);
}
