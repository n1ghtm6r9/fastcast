use crate::web::{WebError, host_with_port};
use tokio::net::TcpListener;

pub async fn bind_listener(host: &str, port: u16) -> Result<TcpListener, WebError> {
    TcpListener::bind((host, port))
        .await
        .map_err(|source| WebError::Bind {
            address: host_with_port(host, port),
            source,
        })
}
