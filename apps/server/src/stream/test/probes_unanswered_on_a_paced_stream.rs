use super::{loopback_stream_socket, probe_of_length, send_with_random_gaps};
use crate::hub::Hub;
use crate::stream::{EndpointOptions, read_datagram, run_receiver};
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn probes_unanswered_on_a_paced_stream(seed: u64, count: usize) -> usize {
    let server = Arc::new(loopback_stream_socket());

    server
        .set_read_timeout(Some(Duration::from_millis(2)))
        .expect("a read timeout can be set");

    let target = server.local_addr().expect("a bound socket has an address");
    let stop = Arc::new(AtomicBool::new(false));
    let receiver = thread::spawn({
        let server = server.clone();
        let stop = stop.clone();

        move || {
            run_receiver(
                server,
                Arc::new(Hub::new()),
                EndpointOptions::new("test-pc".to_owned(), 60),
                stop,
            )
        }
    });

    let prober = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");

    prober
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("a read timeout can be set");

    let probe = probe_of_length(64);
    let mut answered = 0;

    thread::scope(|scope| {
        let sender = scope.spawn(|| send_with_random_gaps(&prober, target, &probe, seed, count));
        let mut buffer = [0u8; 2048];

        while answered < count {
            match read_datagram(&prober, &mut buffer) {
                Ok(_) => answered += 1,
                Err(_) if sender.is_finished() => break,
                Err(_) => {}
            }
        }
    });

    stop.store(true, Ordering::Release);
    receiver.join().expect("the receiver thread does not panic");

    count - answered
}
