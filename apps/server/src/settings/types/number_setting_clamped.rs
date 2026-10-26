use crate::settings::NumberSetting;

impl NumberSetting {
    pub fn clamped(&self, value: u32) -> u32 {
        value.clamp(self.min, self.max)
    }
}
