use crate::control::ReceiveCounters;

#[test]
fn received_packets_count_every_datagram() {
    let mut counters = ReceiveCounters::default();

    for sequence in [5, 6, 6] {
        counters.record(sequence);
    }

    assert_eq!(counters.received_packets, 3);
    assert_eq!(counters.highest_sequence, 6);
}
