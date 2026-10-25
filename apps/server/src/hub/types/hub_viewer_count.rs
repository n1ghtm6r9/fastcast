use crate::hub::Hub;

impl Hub {
    pub fn viewer_count(&self) -> usize {
        *self.viewers.borrow()
    }
}
