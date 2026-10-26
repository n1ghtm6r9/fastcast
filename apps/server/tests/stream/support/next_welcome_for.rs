use crate::support::{TestViewer, next_welcome};
use std::time::{Duration, Instant};

pub fn next_welcome_for(viewer: &TestViewer, nonce: u32, timeout: Duration) -> Option<u32> {
    let deadline = Instant::now() + timeout;

    loop {
        let (session, welcome) =
            next_welcome(viewer, deadline.saturating_duration_since(Instant::now()))?;

        if welcome.nonce == nonce {
            return Some(session);
        }
    }
}
