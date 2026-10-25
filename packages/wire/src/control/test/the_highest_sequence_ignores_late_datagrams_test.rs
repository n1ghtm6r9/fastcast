use crate::control::ReceiveCounters;

#[test]
fn the_highest_sequence_ignores_late_datagrams() {
    let mut counters = ReceiveCounters::default();

    counters.record(10);
    counters.record(3);

    assert_eq!(counters.highest_sequence, 10);
    assert_eq!(counters.received_packets, 2);
}
