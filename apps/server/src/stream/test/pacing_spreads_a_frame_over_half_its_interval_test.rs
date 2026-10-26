use super::{bind_peer, datagrams_of_shards, loopback_stream_socket};
use crate::stream::send_paced;
use std::time::{Duration, Instant};

#[test]
fn pacing_spreads_a_frame_over_half_its_interval() {
    let sender = loopback_stream_socket();
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let datagrams = datagrams_of_shards(64);

    assert_eq!(datagrams.len(), 64);

    let started = Instant::now();

    send_paced(&sender, address, &datagrams, Duration::from_secs(1) / 60);

    let took = started.elapsed();

    assert!(
        took >= Duration::from_millis(7),
        "the frame left in {took:?}, sooner than its last burst is due"
    );
    assert!(
        took < Duration::from_millis(50),
        "the frame took {took:?} to leave"
    );
}
