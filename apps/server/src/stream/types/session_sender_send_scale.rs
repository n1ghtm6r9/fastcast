use crate::display::DisplayScale;
use crate::stream::{SessionSender, scale_notice_json};
use wire::Packet;

impl SessionSender {
    pub fn send_scale(&mut self, scale: &DisplayScale) {
        self.send_control(&Packet::Notice(&scale_notice_json(scale)));
    }
}
