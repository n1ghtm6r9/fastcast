use crate::link::test::datagrams_a_link_loses_on_a_paced_stream;
use std::thread;

#[test]
fn a_link_loses_no_datagram_to_its_read_timeout() {
    let streams: Vec<_> = (0..8)
        .map(|seed| thread::spawn(move || datagrams_a_link_loses_on_a_paced_stream(seed, 400)))
        .collect();

    let lost: u64 = streams
        .into_iter()
        .map(|stream| stream.join().unwrap())
        .sum();

    assert_eq!(lost, 0);
}
