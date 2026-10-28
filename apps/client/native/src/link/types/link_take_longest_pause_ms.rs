use crate::link::Link;

impl Link {
    pub fn take_longest_pause_ms(&self) -> u32 {
        self.shared.take_longest_pause_ms()
    }
}
