#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailedReopen {
    Quiet,
    Log,
    GiveUp,
}
