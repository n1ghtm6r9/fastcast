use crate::server::ServerStatus;
use crate::settings::{SavedSettings, SettingsFileError};
use crate::window::ServerWindow;
use crate::window::test::{WindowProbe, test_window_listing, two_monitors};

pub fn test_window(
    saved: SavedSettings,
    status: ServerStatus,
    saving: fn() -> Result<(), SettingsFileError>,
) -> (ServerWindow<'static>, WindowProbe) {
    test_window_listing(saved, status, saving, two_monitors)
}
