use crate::link::{Link, socket_drops_of};

impl Link {
    pub fn socket_drops(&self) -> Option<u32> {
        socket_drops_of(&self.socket)
    }
}
