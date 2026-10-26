use crate::stream::SessionControl;

impl SessionControl {
    pub async fn woken(&self) {
        self.wake.notified().await;
    }
}
