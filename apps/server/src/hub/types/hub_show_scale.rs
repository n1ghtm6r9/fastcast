use crate::display::DisplayScale;
use crate::hub::Hub;

impl Hub {
    pub fn show_scale(&self, scale: Option<DisplayScale>) {
        self.scale.send_replace(scale);
    }
}
