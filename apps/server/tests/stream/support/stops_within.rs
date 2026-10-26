use app::stream::StreamEndpoint;
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;

pub fn stops_within(endpoint: StreamEndpoint, limit: Duration) -> bool {
    let (stopped, wait) = channel();

    spawn(move || {
        endpoint.stop();

        let _ = stopped.send(());
    });

    wait.recv_timeout(limit).is_ok()
}
