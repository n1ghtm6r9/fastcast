use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn http_get(address: SocketAddr, path: &str) -> String {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("the server accepts connections");
    let request = format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n");

    stream
        .write_all(request.as_bytes())
        .await
        .expect("the request is written");

    let mut answer = String::new();

    timeout(Duration::from_secs(5), stream.read_to_string(&mut answer))
        .await
        .expect("the server answers in time")
        .expect("the answer is text");

    answer
}
