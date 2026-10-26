use crate::journal::{JournalLevel, JournalSink, LocalClock, journal_targets};
use crate::settings::LogLevel;
use tracing::Subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Registry, fmt, reload};

pub fn journal_subscriber(
    sink: JournalSink,
    level: LogLevel,
) -> (impl Subscriber + Send + Sync + 'static, JournalLevel) {
    let (targets, handle) = reload::Layer::new(journal_targets(level));
    let subscriber = Registry::default().with(targets).with(
        fmt::layer()
            .with_writer(sink)
            .with_ansi(false)
            .with_target(false)
            .with_timer(LocalClock),
    );

    (subscriber, JournalLevel { handle })
}
