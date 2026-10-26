use super::{bind_peer, datagrams_of_shards, loopback_stream_socket, next_datagram_bytes};
use crate::stream::send_paced;
use std::thread;
use std::time::Duration;
use wire::decode_datagram;

#[test]
fn paced_datagrams_reach_the_peer_in_order() {
    let sender = loopback_stream_socket();
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let datagrams = datagrams_of_shards(40);

    assert_eq!(datagrams.len(), 40);

    thread::scope(|scope| {
        scope.spawn(|| send_paced(&sender, address, &datagrams, Duration::from_secs(1) / 60));

        let mut sequences = Vec::new();

        for index in 0..datagrams.len() {
            let bytes = next_datagram_bytes(&peer).expect("every datagram arrives");

            assert_eq!(bytes, datagrams.get(index));

            sequences.push(decode_datagram(&bytes).expect("a valid datagram").sequence);
        }

        assert!(sequences.windows(2).all(|pair| pair[0] < pair[1]));
    });
}
