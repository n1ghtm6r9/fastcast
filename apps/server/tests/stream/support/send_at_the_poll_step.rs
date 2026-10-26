use crate::support::{POLL_STEP, spin_for};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn send_at_the_poll_step(
    socket: &UdpSocket,
    target: SocketAddr,
    payload: &[u8],
    seed: u64,
    count: usize,
) {
    let mut state = seed.wrapping_mul(7919).wrapping_add(12345);

    for _ in 0..count {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let jitter = Duration::from_micros((state >> 33) % 500);

        spin_for(POLL_STEP - Duration::from_micros(250) + jitter);
        socket
            .send_to(payload, target)
            .expect("a datagram to the loopback is sent");
    }
}
