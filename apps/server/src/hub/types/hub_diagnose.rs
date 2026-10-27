use crate::hub::Hub;

impl Hub {
    pub fn diagnose(&self, run: u64) {
        self.diagnose.send_replace(Some(run));
    }
}
