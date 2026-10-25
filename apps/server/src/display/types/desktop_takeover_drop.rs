use crate::display::{DesktopTakeover, restore_displays};

impl Drop for DesktopTakeover {
    fn drop(&mut self) {
        drop(self.added.take());
        restore_displays(&self.saved);
    }
}
