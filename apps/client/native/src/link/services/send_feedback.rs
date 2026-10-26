use crate::link::{LinkShared, LinkState, LinkTarget, send_packet};
use std::net::UdpSocket;
use std::time::Instant;
use wire::{FEEDBACK_INTERVAL, Feedback, Packet};

pub(crate) fn send_feedback(
    socket: &UdpSocket,
    target: &LinkTarget,
    shared: &LinkShared,
    state: &mut LinkState,
    now: Instant,
) {
    if state.session.is_none() {
        return;
    }

    let lost_frames = state.receiver.lost_frames();

    if now < state.next_feedback && lost_frames <= state.reported_losses {
        return;
    }

    let feedback = Feedback {
        highest_frame: state.receiver.highest_frame(),
        received_packets: state.counters.received_packets,
        highest_sequence: state.counters.highest_sequence,
        lost_frames,
        recovered_shards: state.receiver.recovered_shards(),
        keyframe_needed: state.receiver.keyframe_needed() || shared.queue.needs_key_frame(),
    };

    send_packet(socket, target.server, shared, Packet::Feedback(feedback));

    state.reported_losses = lost_frames;
    state.next_feedback = now + FEEDBACK_INTERVAL;
}
