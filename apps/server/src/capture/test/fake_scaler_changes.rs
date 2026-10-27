use super::FakeScaler;

impl FakeScaler {
    pub fn changes(&self) -> Vec<(u32, i32)> {
        self.changes
            .lock()
            .expect("the changes lock is not poisoned")
            .clone()
    }
}
