use crate::display::KeepAlive;

impl Drop for KeepAlive {
    fn drop(&mut self) {
        drop(self.stop.take());

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
