use crate::settings::Settings;
use tokio::sync::watch;

pub async fn wait_for_other_settings(
    settings: &mut watch::Receiver<Settings>,
    current: &Settings,
) -> bool {
    settings.wait_for(|latest| latest != current).await.is_ok()
}
