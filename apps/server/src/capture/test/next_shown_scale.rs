use crate::display::DisplayScale;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::timeout;

pub async fn next_shown_scale(
    shown: &mut watch::Receiver<Option<DisplayScale>>,
) -> Option<DisplayScale> {
    timeout(Duration::from_secs(2), shown.changed())
        .await
        .expect("a scale is shown in time")
        .expect("the hub is alive");

    *shown.borrow_and_update()
}
