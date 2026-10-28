use super::{bind_peer, frame_under, loopback_stream_socket};
use crate::fixtures::{config_of, x264_1920x1080_avcc};
use crate::stream::{SessionControl, SessionSender};
use std::sync::Arc;

#[test]
fn a_frame_without_a_screen_time_counts_nothing() {
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let control = Arc::new(SessionControl::new());
    let socket = Arc::new(loopback_stream_socket());
    let mut sender = SessionSender::new(socket, address, 1, control.clone());
    let config = Arc::new(config_of(&x264_1920x1080_avcc(), 60));

    sender.send_video(&frame_under(&config, true));

    assert!(control.take_sent().is_empty());
}
