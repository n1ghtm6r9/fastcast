use crate::settings::FrameRate;

impl FrameRate {
    pub fn limit(self) -> Option<u32> {
        match self {
            FrameRate::Fps24 => Some(24),
            FrameRate::Fps30 => Some(30),
            FrameRate::Fps60 => Some(60),
            FrameRate::Fps120 => Some(120),
            FrameRate::Fps240 => Some(240),
            FrameRate::Unlimited => None,
        }
    }
}
