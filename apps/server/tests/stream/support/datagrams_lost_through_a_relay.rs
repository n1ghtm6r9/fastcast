use crate::support::{POLL_STEP, bind_peer, lossy_relay, receive_within, send_at_the_poll_step};
use std::thread;
use std::time::{Duration, Instant};

pub fn datagrams_lost_through_a_relay(seed: u64, count: usize) -> usize {
    let server = bind_peer();
    let relay = lossy_relay(
        server.local_addr().expect("a bound socket has an address"),
        usize::MAX,
    );
    let viewer = bind_peer();
    let relay_address = relay.local_addr();

    viewer
        .send_to(&[0u8; 8], relay_address)
        .expect("the first datagram is sent");
    receive_within(&server, Duration::from_secs(2)).expect("the relay forwards the first datagram");

    let sender = thread::spawn(move || {
        send_at_the_poll_step(&server, relay_address, &[0u8; 8], seed, count);
    });

    let mut received = 0;
    let mut last_arrival = Instant::now();

    while received < count
        && !(sender.is_finished() && last_arrival.elapsed() > Duration::from_secs(1))
    {
        if receive_within(&viewer, POLL_STEP).is_some() {
            received += 1;
            last_arrival = Instant::now();
        }
    }

    sender.join().expect("the sender thread does not panic");

    count - received
}
