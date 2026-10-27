use crate::display::ScaleLevels;
use std::sync::{Arc, Mutex};

pub struct FakeScaler {
    pub(super) output: u32,
    pub(super) levels: Mutex<Option<ScaleLevels>>,
    pub(super) refuses: bool,
    pub(super) changes: Mutex<Vec<(u32, i32)>>,
}

impl FakeScaler {
    pub fn new(output: u32, levels: Option<ScaleLevels>, refuses: bool) -> Arc<Self> {
        Arc::new(Self {
            output,
            levels: Mutex::new(levels),
            refuses,
            changes: Mutex::new(Vec::new()),
        })
    }
}
