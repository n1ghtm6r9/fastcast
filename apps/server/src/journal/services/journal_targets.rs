use crate::journal::{FASTCAST_TARGETS, OTHER_CRATES_LEVEL, level_filter_of};
use crate::settings::LogLevel;
use tracing_subscriber::filter::Targets;

pub fn journal_targets(level: LogLevel) -> Targets {
    Targets::new()
        .with_targets(
            FASTCAST_TARGETS
                .into_iter()
                .map(|target| (target, level_filter_of(level))),
        )
        .with_default(OTHER_CRATES_LEVEL)
}
