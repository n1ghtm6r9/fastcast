use crate::server::ServerStatus;
use crate::window::ServerLink;

impl ServerLink<'_> {
    pub fn status(&self) -> ServerStatus {
        self.status.borrow().clone()
    }
}
