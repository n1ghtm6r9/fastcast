use crate::capture::Screen;
use tokio::task::spawn_blocking;

pub async fn release_screen(screen: Option<Screen>) {
    if let Some(virtual_display) = screen.and_then(|screen| screen.display) {
        let _ = spawn_blocking(move || drop(virtual_display)).await;
    }
}
