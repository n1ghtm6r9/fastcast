use super::ScriptedProbe;
use crate::settings::VideoEncoder;

impl ScriptedProbe {
    pub fn calls(&self) -> Vec<VideoEncoder> {
        self.calls
            .lock()
            .expect("the calls lock is not poisoned")
            .clone()
    }
}
