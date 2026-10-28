use crate::capture::CaptureStats;

impl CaptureStats {
    pub fn replaced(&mut self) {
        self.replaced += 1;
    }
}
