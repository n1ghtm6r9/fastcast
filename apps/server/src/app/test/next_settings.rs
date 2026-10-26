use crate::app::test::ANSWER_LIMIT;
use crate::settings::Settings;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::timeout;

pub async fn next_settings(started: &mut UnboundedReceiver<Settings>) -> Settings {
    timeout(ANSWER_LIMIT, started.recv())
        .await
        .expect("the server starts in time")
        .expect("the control is running")
}
