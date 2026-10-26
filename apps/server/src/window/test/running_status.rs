use crate::server::{ServerState, ServerStatus};
use crate::window::test::TEST_ADDRESS;

pub fn running_status() -> ServerStatus {
    ServerStatus {
        state: ServerState::Running {
            address: TEST_ADDRESS,
        },
        ..ServerStatus::default()
    }
}
