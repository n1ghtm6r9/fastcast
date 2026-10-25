use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn websocket_handshake(address: SocketAddr, path: &str, extra_headers: &str) -> String {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("the server accepts connections");
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n{extra_headers}\r\n"
    );

    stream
        .write_all(request.as_bytes())
        .await
        .expect("the handshake is written");

    let mut answer = Vec::new();
    let mut chunk = [0u8; 1024];

    while !answer.windows(4).any(|window| window == b"\r\n\r\n") {
        let read = timeout(Duration::from_secs(5), stream.read(&mut chunk))
            .await
            .expect("the server answers in time")
            .expect("the answer can be read");

        assert_ne!(read, 0, "the server closed before it answered");
        answer.extend_from_slice(&chunk[..read]);
    }

    String::from_utf8_lossy(&answer).into_owned()
}
