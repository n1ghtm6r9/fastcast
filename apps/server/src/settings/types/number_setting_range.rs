use crate::settings::NumberSetting;
use std::ops::RangeInclusive;

impl NumberSetting {
    pub fn range(&self) -> RangeInclusive<u32> {
        self.min..=self.max
    }
}
