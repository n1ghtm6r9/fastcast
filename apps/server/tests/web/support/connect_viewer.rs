use crate::support::Viewer;
use std::net::SocketAddr;
use tokio_tungstenite::connect_async;

pub async fn connect_viewer(address: SocketAddr) -> Viewer {
    let (viewer, _) = connect_async(format!("ws://{address}/ws"))
        .await
        .expect("the websocket handshake succeeds");

    viewer
}
