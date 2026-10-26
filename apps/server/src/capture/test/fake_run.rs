use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum FakeRun {
    Forever,
    EndsAfter { delay: Duration, frames: u64 },
    Fails { delay: Duration },
    FailsNatively { frames: u64 },
    FailsNativelyTemporarily { frames: u64 },
}
