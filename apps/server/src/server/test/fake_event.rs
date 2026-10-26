use crate::server::ServerStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FakeEvent {
    Started { name: String, status: ServerStatus },
    Stopped { name: String },
}
