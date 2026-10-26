use crate::journal::local_time_of_day;
use std::fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

#[derive(Debug, Clone, Copy, Default)]
pub struct LocalClock;

impl FormatTime for LocalClock {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        let now = local_time_of_day();

        write!(
            writer,
            "{:02}:{:02}:{:02}.{:03}",
            now.hour, now.minute, now.second, now.millisecond
        )
    }
}
