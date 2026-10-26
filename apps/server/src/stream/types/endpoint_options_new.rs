use crate::stream::EndpointOptions;
use wire::SESSION_TIMEOUT;

impl EndpointOptions {
    pub fn new(server_name: String, default_fps: u32) -> EndpointOptions {
        EndpointOptions {
            server_name,
            default_fps,
            session_timeout: SESSION_TIMEOUT,
        }
    }
}
