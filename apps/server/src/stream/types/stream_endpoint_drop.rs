use crate::stream::StreamEndpoint;

impl Drop for StreamEndpoint {
    fn drop(&mut self) {
        self.shutdown();
    }
}
