use crate::server::ServerControl;
use crate::settings::Settings;

impl ServerControl {
    pub fn apply(&self, settings: Settings) {
        self.settings.send_replace(settings);
    }
}
