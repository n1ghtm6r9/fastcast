use crate::link::Link;

impl Drop for Link {
    fn drop(&mut self) {
        self.close();
    }
}
