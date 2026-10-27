use crate::hub::Hub;

impl Hub {
    pub fn ask_scale(&self, step: i32) {
        let _ = self.scale_asks.send(step);
    }
}
