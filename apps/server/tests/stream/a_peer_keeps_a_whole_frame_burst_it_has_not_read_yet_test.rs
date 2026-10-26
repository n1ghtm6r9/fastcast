use crate::support::{bind_peer, receive_within};
use std::time::Duration;

#[test]
fn a_peer_keeps_a_whole_frame_burst_it_has_not_read_yet() {
    let receiver = bind_peer();
    let sender = bind_peer();
    let target = receiver
        .local_addr()
        .expect("a bound socket has an address");

    for _ in 0..250 {
        sender
            .send_to(&[0u8; 1400], target)
            .expect("a datagram to the loopback is sent");
    }

    let mut received = 0;

    while receive_within(&receiver, Duration::from_millis(100)).is_some() {
        received += 1;
    }

    assert_eq!(received, 250);
}
