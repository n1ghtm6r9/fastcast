use crate::link::{LinkShared, LinkTarget, end_link_on_panic, raise_drain_priority, run_link};
use std::net::UdpSocket;
use std::sync::Arc;

pub(crate) fn link_thread(socket: Arc<UdpSocket>, target: LinkTarget, shared: Arc<LinkShared>) {
    raise_drain_priority();
    let running = Arc::clone(&shared);

    end_link_on_panic(&shared, move || run_link(socket, target, running));
}
