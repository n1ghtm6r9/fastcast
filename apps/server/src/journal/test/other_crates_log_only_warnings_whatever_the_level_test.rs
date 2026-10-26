use crate::journal::journal_targets;
use crate::settings::LogLevel;
use tracing::Level;

#[test]
fn other_crates_log_only_warnings_whatever_the_level() {
    let targets = journal_targets(LogLevel::Trace);

    assert!(!targets.would_enable("winit::platform_impl", &Level::INFO));
    assert!(targets.would_enable("egui_glow::painter", &Level::WARN));
}
