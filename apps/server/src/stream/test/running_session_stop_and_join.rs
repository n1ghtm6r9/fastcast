use super::RunningSession;
use std::net::UdpSocket;

impl RunningSession {
    pub fn stop_and_join(self) -> UdpSocket {
        self.control.stop();
        self.thread
            .join()
            .expect("the session thread does not panic");

        self.peer
    }
}
