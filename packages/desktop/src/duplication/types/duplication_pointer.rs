use crate::duplication::{Duplication, PointerState};

impl Duplication {
    pub fn pointer(&self) -> &PointerState {
        &self.pointer
    }
}
