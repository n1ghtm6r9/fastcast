use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type Viewer = WebSocketStream<MaybeTlsStream<TcpStream>>;
