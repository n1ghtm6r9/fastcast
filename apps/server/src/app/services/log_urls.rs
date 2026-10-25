use crate::app::{lan_ip, urls_to_open};
use tracing::info;

pub fn log_urls(host: &str, port: u16) {
    for url in urls_to_open(host, port, lan_ip()) {
        info!("fastcast is listening on {url}");
    }

    info!("stream endpoint /ws, health check /api/health");
}
