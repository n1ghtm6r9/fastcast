use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EndpointOptions {
    pub server_name: String,
    pub default_fps: u32,
    pub session_timeout: Duration,
}
