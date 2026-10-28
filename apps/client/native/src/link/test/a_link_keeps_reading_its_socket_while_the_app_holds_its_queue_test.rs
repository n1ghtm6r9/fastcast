use crate::link::test::{connected_link, patterned_bytes};
use std::thread::sleep;
use std::time::Duration;
use wire::{FrameKind, decode_datagram};

#[test]
fn a_link_keeps_reading_its_socket_while_the_app_holds_its_queue() {
    let (mut server, link, peer) = connected_link();
    let frame = patterned_bytes(120_000);
    let frames = link.receive_buffer_bytes as usize * 3 / 2 / frame.len() + 1;
    let held = link.shared.queue.lock();
    let mut last_sequence = 0;

    for _ in 0..frames {
        let datagrams = server.send_frame(peer, FrameKind::Video, true, &frame, 0);

        last_sequence = decode_datagram(datagrams.last().unwrap()).unwrap().sequence;
        sleep(Duration::from_millis(1));
    }

    drop(held);

    let feedback = server.last_feedback_reaching(last_sequence);

    assert_eq!(feedback.received_packets, last_sequence + 1);
}
