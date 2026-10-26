use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ServerState {
    #[default]
    Starting,
    Running {
        address: SocketAddr,
    },
    Failed {
        reason: String,
    },
    Stopped,
}
