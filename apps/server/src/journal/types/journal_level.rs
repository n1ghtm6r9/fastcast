use tracing_subscriber::filter::Targets;
use tracing_subscriber::{Registry, reload};

#[derive(Clone)]
pub struct JournalLevel {
    pub(crate) handle: reload::Handle<Targets, Registry>,
}
