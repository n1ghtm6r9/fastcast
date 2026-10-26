use crate::journal::journal_targets;
use crate::settings::LogLevel;
use tracing::Level;

#[test]
fn the_chosen_level_applies_to_every_fastcast_crate() {
    let targets = journal_targets(LogLevel::Debug);

    for target in [
        "app::capture",
        "fastcast",
        "desktop",
        "nvenc",
        "media",
        "wire",
    ] {
        assert!(targets.would_enable(target, &Level::DEBUG), "{target}");
        assert!(!targets.would_enable(target, &Level::TRACE), "{target}");
    }
}
