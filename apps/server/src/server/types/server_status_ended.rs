use crate::server::{ServerState, ServerStatus};
use crate::stream::StreamError;

impl ServerStatus {
    pub fn ended(ended: &Result<(), StreamError>) -> ServerStatus {
        let state = match ended {
            Ok(()) => ServerState::Stopped,
            Err(failure) => ServerState::Failed {
                reason: failure.to_string(),
            },
        };

        ServerStatus {
            state,
            ..ServerStatus::default()
        }
    }
}
