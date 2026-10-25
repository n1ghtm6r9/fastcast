use super::FakeBackend;
use crate::hub::ViewerRequest;

impl FakeBackend {
    pub fn opened(&self) -> Vec<ViewerRequest> {
        self.opened
            .lock()
            .expect("the opened lock is not poisoned")
            .clone()
    }
}
