use crate::link::{
    Datagram, LinkShared, RECEIVE_BUFFER_BYTES, end_link, is_quiet_read_error, note_turn,
    read_datagram,
};
use std::net::UdpSocket;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::time::Instant;

pub(crate) fn drain_socket(
    socket: &UdpSocket,
    shared: &LinkShared,
    inbox: SyncSender<Datagram>,
    serving_ended: impl Fn() -> bool,
) {
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];
    let mut last_turn = Instant::now();

    while !shared.stop.load(Ordering::Acquire) && !serving_ended() {
        note_turn(shared, &mut last_turn, Instant::now());

        match read_datagram(socket, &mut buffer) {
            Ok((length, from)) => {
                let datagram = Datagram {
                    bytes: buffer[..length].to_vec(),
                    from,
                };

                match inbox.try_send(datagram) {
                    Ok(()) => {}
                    Err(TrySendError::Full(_)) => {
                        shared.inbox_drops.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(TrySendError::Disconnected(_)) => return,
                }
            }
            Err(error) if is_quiet_read_error(&error) => {}
            Err(error) => {
                end_link(shared, &error.to_string());
                return;
            }
        }
    }
}
