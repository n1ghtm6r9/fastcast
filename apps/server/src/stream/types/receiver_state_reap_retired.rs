use crate::stream::{ReceiverState, join_session_thread};
use std::mem::take;
use std::thread::JoinHandle;

impl ReceiverState {
    pub fn reap_retired(&mut self) {
        let (finished, running): (Vec<_>, Vec<_>) = take(&mut self.retired)
            .into_iter()
            .partition(JoinHandle::is_finished);

        self.retired = running;

        for thread in finished {
            join_session_thread(thread);
        }
    }
}
