use crate::support::TestViewer;
use wire::Packet;

impl TestViewer {
    pub fn send_feedback(&mut self, keyframe_needed: bool) {
        let feedback = self.feedback(keyframe_needed);

        self.send(&Packet::Feedback(feedback));
    }
}
