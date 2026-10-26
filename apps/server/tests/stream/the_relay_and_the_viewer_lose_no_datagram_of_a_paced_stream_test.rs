use crate::support::datagrams_lost_through_a_relay;
use std::thread;

#[test]
fn the_relay_and_the_viewer_lose_no_datagram_of_a_paced_stream() {
    let streams: Vec<_> = (0..4)
        .map(|seed| thread::spawn(move || datagrams_lost_through_a_relay(seed, 250)))
        .collect();

    let lost: usize = streams
        .into_iter()
        .map(|stream| stream.join().expect("the stream thread does not panic"))
        .sum();

    assert_eq!(lost, 0);
}
