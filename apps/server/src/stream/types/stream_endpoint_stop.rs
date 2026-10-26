use crate::stream::StreamEndpoint;

impl StreamEndpoint {
    pub fn stop(mut self) {
        self.shutdown();
    }
}
