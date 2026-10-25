use crate::support::Viewer;
use std::net::SocketAddr;
use tokio_tungstenite::connect_async;

pub async fn connect_viewer_asking(address: SocketAddr, query: &str) -> Viewer {
    let (viewer, _) = connect_async(format!("ws://{address}/ws?{query}"))
        .await
        .expect("the websocket handshake succeeds");

    viewer
}
