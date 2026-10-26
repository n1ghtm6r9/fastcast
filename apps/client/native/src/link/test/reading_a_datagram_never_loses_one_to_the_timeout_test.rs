use crate::link::test::datagrams_lost_on_a_paced_stream;
use std::thread;

#[test]
fn reading_a_datagram_never_loses_one_to_the_timeout() {
    let streams: Vec<_> = (0..8)
        .map(|seed| thread::spawn(move || datagrams_lost_on_a_paced_stream(seed, 400)))
        .collect();

    let lost: usize = streams
        .into_iter()
        .map(|stream| stream.join().unwrap())
        .sum();

    assert_eq!(lost, 0);
}
