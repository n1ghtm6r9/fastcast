use super::FakeScaler;
use std::sync::Arc;

impl FakeScaler {
    pub fn unknown() -> Arc<Self> {
        Self::new(0, None, false)
    }
}
