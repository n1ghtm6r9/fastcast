use crate::control::test::feedback;
use crate::control::{LossMeter, ReceiveCounters};

#[test]
fn loss_is_measured_across_a_sequence_wrap() {
    let mut counters = ReceiveCounters {
        received_packets: 0,
        highest_sequence: u32::MAX - 10,
    };
    let mut meter = LossMeter::new();

    for sequence in u32::MAX - 9..=u32::MAX {
        counters.record(sequence);
    }
    meter.update(&feedback(
        counters.received_packets,
        counters.highest_sequence,
    ));

    for sequence in (0..10).filter(|&sequence| sequence != 3) {
        counters.record(sequence);
    }
    let loss = meter
        .update(&feedback(
            counters.received_packets,
            counters.highest_sequence,
        ))
        .expect("a loss figure");

    assert!((loss - 0.1).abs() < 1e-9);
}
