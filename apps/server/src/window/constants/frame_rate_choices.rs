use crate::settings::FrameRate;

pub const FRAME_RATE_CHOICES: [(FrameRate, &str); 6] = [
    (FrameRate::Fps24, "24"),
    (FrameRate::Fps30, "30"),
    (FrameRate::Fps60, "60"),
    (FrameRate::Fps120, "120"),
    (FrameRate::Fps240, "240"),
    (FrameRate::Unlimited, "Без ограничения"),
];
