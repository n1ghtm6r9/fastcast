use super::probes_unanswered_on_a_paced_stream;
use std::thread;

#[test]
fn the_receiver_answers_every_probe_of_a_paced_stream() {
    let streams: Vec<_> = (0..8)
        .map(|seed| thread::spawn(move || probes_unanswered_on_a_paced_stream(seed, 400)))
        .collect();

    let unanswered: usize = streams
        .into_iter()
        .map(|stream| stream.join().expect("the stream thread does not panic"))
        .sum();

    assert_eq!(unanswered, 0);
}
