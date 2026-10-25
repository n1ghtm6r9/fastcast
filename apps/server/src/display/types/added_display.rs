use crate::display::{DriverHandle, KeepAlive};
use std::sync::Arc;

pub struct AddedDisplay {
    pub(crate) _keep_alive: KeepAlive,
    pub(crate) driver: Arc<DriverHandle>,
    pub(crate) index: u16,
}
