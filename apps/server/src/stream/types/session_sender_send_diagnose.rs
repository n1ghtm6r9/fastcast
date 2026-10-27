use crate::stream::{SessionSender, diagnose_notice_json};
use wire::Packet;

impl SessionSender {
    pub fn send_diagnose(&mut self, run: u64) {
        self.send_control(&Packet::Notice(&diagnose_notice_json(run)));
    }
}
