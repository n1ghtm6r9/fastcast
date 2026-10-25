use crate::discovery::{RECEIVE_BUFFER_BYTES, answer_datagram, server_reply_json};
use tokio::net::UdpSocket;
use tracing::debug;

pub async fn serve_discovery(socket: UdpSocket, name: String, web_port: u16) {
    let reply = server_reply_json(&name, web_port);
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];

    loop {
        match socket.recv_from(&mut buffer).await {
            Ok((length, sender)) => {
                answer_datagram(&socket, &buffer[..length], sender, &reply).await
            }
            Err(error) => debug!("could not read a discovery datagram: {error}"),
        }
    }
}
