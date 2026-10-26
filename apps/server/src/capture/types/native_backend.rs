use crate::settings::Settings;
use std::sync::Mutex;
use std::thread::JoinHandle;

pub struct NativeBackend {
    pub(crate) settings: Settings,
    pub(crate) worker: Mutex<Option<JoinHandle<()>>>,
}

impl NativeBackend {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            worker: Mutex::new(None),
        }
    }
}
