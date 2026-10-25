use std::sync::Arc;
use tokio::sync::watch;

pub struct ViewerGuard {
    viewers: Arc<watch::Sender<usize>>,
}

impl ViewerGuard {
    pub fn new(viewers: Arc<watch::Sender<usize>>) -> Self {
        viewers.send_modify(|count| *count += 1);

        Self { viewers }
    }
}

impl Drop for ViewerGuard {
    fn drop(&mut self) {
        self.viewers
            .send_modify(|count| *count = count.saturating_sub(1));
    }
}
