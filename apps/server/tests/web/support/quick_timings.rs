use app::web::ViewerTimings;
use std::time::Duration;

pub fn quick_timings() -> ViewerTimings {
    ViewerTimings {
        ping_interval: Duration::from_millis(50),
        silence_timeout: Duration::from_millis(600),
        write_timeout: Duration::from_millis(600),
    }
}
