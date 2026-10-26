use std::thread::JoinHandle;
use tracing::warn;

pub fn join_session_thread(thread: JoinHandle<()>) {
    if thread.join().is_err() {
        warn!("a stream session thread panicked");
    }
}
