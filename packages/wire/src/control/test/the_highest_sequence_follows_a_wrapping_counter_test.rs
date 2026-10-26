use crate::control::ReceiveCounters;

#[test]
fn the_highest_sequence_follows_a_wrapping_counter() {
    let mut counters = ReceiveCounters::default();

    for sequence in [u32::MAX - 1, u32::MAX, 0, 2, u32::MAX] {
        counters.record(sequence);
    }

    assert_eq!(counters.highest_sequence, 2);
    assert_eq!(counters.received_packets, 5);
}
