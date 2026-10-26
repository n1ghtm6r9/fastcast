use crate::settings::LogLevel;
use tracing::level_filters::LevelFilter;

pub fn level_filter_of(level: LogLevel) -> LevelFilter {
    match level {
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Trace => LevelFilter::TRACE,
    }
}
