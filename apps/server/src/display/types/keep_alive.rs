use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub struct KeepAlive {
    pub(crate) stop: Option<Sender<()>>,
    pub(crate) thread: Option<JoinHandle<()>>,
}
