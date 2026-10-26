use std::io;
use std::thread::{self, JoinHandle};

pub fn spawn_capture_worker(
    previous: Option<JoinHandle<()>>,
    work: impl FnOnce() + Send + 'static,
) -> io::Result<JoinHandle<()>> {
    thread::Builder::new()
        .name("fastcast-capture".to_owned())
        .spawn(move || {
            if let Some(previous) = previous {
                let _ = previous.join();
            }

            work();
        })
}
