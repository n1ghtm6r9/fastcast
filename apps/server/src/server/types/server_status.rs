use crate::hub::ViewerRequest;
use crate::server::ServerState;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServerStatus {
    pub state: ServerState,
    pub viewers: usize,
    pub request: ViewerRequest,
    pub report: Option<String>,
}
