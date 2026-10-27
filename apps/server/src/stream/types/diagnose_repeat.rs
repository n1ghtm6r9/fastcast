use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct DiagnoseRepeat {
    pub(crate) run: Option<u64>,
    pub(crate) sent: Option<Instant>,
}

impl DiagnoseRepeat {
    pub fn new(run: Option<u64>) -> Self {
        Self { run, sent: None }
    }
}
