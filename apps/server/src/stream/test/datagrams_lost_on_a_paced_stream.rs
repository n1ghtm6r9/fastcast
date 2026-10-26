use super::{loopback_stream_socket, send_with_random_gaps};
use crate::stream::read_datagram;
use std::net::{Ipv4Addr, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

pub fn datagrams_lost_on_a_paced_stream(seed: u64, count: usize) -> usize {
    let receiver = loopback_stream_socket();

    receiver
        .set_read_timeout(Some(Duration::from_millis(2)))
        .expect("a read timeout can be set");

    let target = receiver
        .local_addr()
        .expect("a bound socket has an address");

    let sender = thread::spawn(move || {
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");

        send_with_random_gaps(&socket, target, &[0u8; 8], seed, count);
    });

    let mut received = 0;
    let mut buffer = [0u8; 64];
    let mut last_arrival = Instant::now();

    while received < count
        && !(sender.is_finished() && last_arrival.elapsed() > Duration::from_secs(1))
    {
        if let Ok((8, _)) = read_datagram(&receiver, &mut buffer) {
            received += 1;
            last_arrival = Instant::now();
        }
    }

    sender.join().expect("the sender thread does not panic");

    count - received
}
