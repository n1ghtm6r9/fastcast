use crate::stream::SessionSender;
use wire::Packet;

impl SessionSender {
    pub fn send_bye(&mut self) {
        self.send_control(&Packet::Bye);
    }
}
