use crate::hub::{Hub, ViewerRequest};

impl Hub {
    pub fn request(&self, request: ViewerRequest) -> bool {
        self.requests.send_if_modified(|current| {
            if *current == request {
                return false;
            }

            *current = request;

            true
        })
    }
}
