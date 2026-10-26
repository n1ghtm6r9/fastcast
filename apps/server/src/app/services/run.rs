use crate::app::{data_directory, init_logger, shutdown_signal};
use crate::server::ServerControl;
use crate::settings::constants::SETTINGS_FILE_NAME;
use crate::settings::{SavedSettings, computer_name, read_saved_settings, settings_of};
use tokio::runtime::Handle;
use tracing::warn;

pub async fn run() {
    init_logger();

    let saved =
        read_saved_settings(&data_directory().join(SETTINGS_FILE_NAME)).unwrap_or_else(|problem| {
            warn!("{problem}, the defaults are used");
            SavedSettings::default()
        });
    let control = ServerControl::start(&Handle::current(), settings_of(&saved, &computer_name()));

    shutdown_signal().await;
    control.stop().await;
}
