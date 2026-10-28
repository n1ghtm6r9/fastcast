use super::{bind_peer, datagrams_of_shards, loopback_stream_socket};
use crate::stream::send_paced;
use std::time::{Duration, Instant};

#[test]
fn a_small_frame_leaves_without_waiting_for_half_its_interval() {
    let sender = loopback_stream_socket();
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let datagrams = datagrams_of_shards(40);

    let started = Instant::now();

    send_paced(&sender, address, &datagrams, Duration::from_secs(1) / 60);

    let took = started.elapsed();

    assert!(
        took < Duration::from_millis(5),
        "a frame of {} datagrams took {took:?} to leave",
        datagrams.len()
    );
}
