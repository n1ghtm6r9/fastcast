use super::Responder;
use crate::discovery::{bind_discovery_socket, serve_discovery};
use std::net::{Ipv4Addr, SocketAddr};

pub async fn start_responder(name: &str, web_port: u16) -> Responder {
    let socket = bind_discovery_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))
        .await
        .expect("a free port exists");
    let address = socket.local_addr().expect("a bound socket has an address");
    let task = tokio::spawn(serve_discovery(socket, name.to_owned(), web_port));

    Responder { address, task }
}
