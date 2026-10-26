use crate::link::{LinkShared, LinkTarget, raise_thread_priority, run_link};
use std::net::UdpSocket;
use std::sync::Arc;

pub(crate) fn link_thread(socket: Arc<UdpSocket>, target: LinkTarget, shared: Arc<LinkShared>) {
    raise_thread_priority();
    run_link(socket, target, shared);
}
