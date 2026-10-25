use crate::packet::{Datagram, Feedback, Packet, encode_datagram};

#[test]
fn the_keyframe_request_is_bit_zero_of_the_feedback_flags() {
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 1,
            sequence: 1,
            packet: Packet::Feedback(Feedback {
                keyframe_needed: true,
                ..Feedback::default()
            }),
        },
        &mut out,
    );

    assert_eq!(out.len(), 36);
    assert_eq!(out[32..], [1, 0, 0, 0]);
}
