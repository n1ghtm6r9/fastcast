use crate::stream::wait_until;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use tracing::debug;
use wire::{Datagrams, PACE_TICK, burst_sizes};

pub fn send_paced(
    socket: &UdpSocket,
    peer: SocketAddr,
    datagrams: &Datagrams,
    frame_interval: Duration,
) {
    let start = Instant::now();
    let mut next = 0;

    for (burst, size) in burst_sizes(datagrams.len(), frame_interval)
        .into_iter()
        .enumerate()
    {
        wait_until(start + PACE_TICK * burst as u32);

        for _ in 0..size {
            if let Err(error) = socket.send_to(datagrams.get(next), peer) {
                debug!(%peer, "a stream datagram was not sent: {error}");
            }

            next += 1;
        }
    }
}
