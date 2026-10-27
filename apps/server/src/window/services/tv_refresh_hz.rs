use crate::server::ServerStatus;

pub fn tv_refresh_hz(status: &ServerStatus) -> Option<u32> {
    status
        .request
        .mode
        .filter(|_| status.viewers > 0)
        .map(|mode| mode.refresh_hz)
}
