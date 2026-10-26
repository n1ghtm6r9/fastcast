use crate::support::TestViewer;
use wire::Feedback;

impl TestViewer {
    pub fn feedback(&self, keyframe_needed: bool) -> Feedback {
        Feedback {
            highest_frame: self.receiver.highest_frame(),
            received_packets: self.counters.received_packets,
            highest_sequence: self.counters.highest_sequence,
            lost_frames: self.receiver.lost_frames(),
            recovered_shards: self.receiver.recovered_shards(),
            keyframe_needed,
        }
    }
}
