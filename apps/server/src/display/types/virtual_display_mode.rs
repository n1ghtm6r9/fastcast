use crate::display::{DisplayMode, VirtualDisplay};

impl VirtualDisplay {
    pub fn mode(&self) -> DisplayMode {
        self.mode
    }
}
