use crate::journal::JournalLines;
use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::watch;

pub struct WindowProbe {
    pub applied: Rc<RefCell<Vec<SavedSettings>>>,
    pub status: watch::Sender<ServerStatus>,
    pub lines: Arc<JournalLines>,
}
