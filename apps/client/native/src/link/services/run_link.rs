use crate::link::{
    INBOX_DATAGRAMS, LINK_THREAD_PRIORITY, LinkShared, LinkTarget, drain_socket, end_link,
    raise_thread_priority, serve_datagrams,
};
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::thread;

pub fn run_link(socket: Arc<UdpSocket>, target: LinkTarget, shared: Arc<LinkShared>) {
    let (inbox, datagrams) = sync_channel(INBOX_DATAGRAMS);

    thread::scope(|scope| {
        let serving = thread::Builder::new()
            .name("fastcast-frames".to_string())
            .spawn_scoped(scope, || {
                raise_thread_priority(LINK_THREAD_PRIORITY);
                serve_datagrams(&socket, &target, &shared, datagrams);
            });

        match serving {
            Ok(serving) => drain_socket(&socket, &shared, inbox, || serving.is_finished()),
            Err(error) => end_link(&shared, &error.to_string()),
        }
    });
}
