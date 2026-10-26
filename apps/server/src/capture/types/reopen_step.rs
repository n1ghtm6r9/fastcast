#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReopenStep {
    Retry,
    Resume,
    Restart,
}
