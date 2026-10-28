use crate::link::{LINK_TICK, LinkShared, drain_socket};
use std::net::UdpSocket;
use std::sync::atomic::Ordering;
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};

#[test]
fn a_full_inbox_counts_the_datagrams_it_throws_away() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket.set_read_timeout(Some(LINK_TICK)).unwrap();
    let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
    let shared = LinkShared::default();
    let (inbox, datagrams) = sync_channel(1);

    for _ in 0..3 {
        sender.send_to(&[7], socket.local_addr().unwrap()).unwrap();
    }

    let until = Instant::now() + Duration::from_millis(200);
    drain_socket(&socket, &shared, inbox, || Instant::now() > until);

    assert_eq!(datagrams.try_iter().count(), 1);
    assert_eq!(shared.inbox_drops.load(Ordering::Relaxed), 2);
}
