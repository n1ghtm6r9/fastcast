use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum FakePreparation {
    Succeeds,
    Takes { delay: Duration },
    Fails,
}
