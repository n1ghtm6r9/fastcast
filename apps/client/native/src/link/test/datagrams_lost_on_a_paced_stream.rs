use crate::link::test::spin_for;
use crate::link::{LINK_TICK, read_datagram};
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) fn datagrams_lost_on_a_paced_stream(seed: u64, count: usize) -> usize {
    let receiver = UdpSocket::bind("0.0.0.0:0").unwrap();
    receiver.set_read_timeout(Some(LINK_TICK)).unwrap();
    let target = SocketAddr::from(([127, 0, 0, 1], receiver.local_addr().unwrap().port()));

    let sender = thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let mut state = seed.wrapping_mul(7919).wrapping_add(12345);

        for _ in 0..count {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            spin_for(Duration::from_micros((state >> 33) % 4000));
            socket.send_to(&[0u8; 8], target).unwrap();
        }
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

    sender.join().unwrap();

    count - received
}
