use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FrameRate {
    #[serde(rename = "24")]
    Fps24,
    #[serde(rename = "30")]
    Fps30,
    #[serde(rename = "60")]
    Fps60,
    #[serde(rename = "120")]
    Fps120,
    #[serde(rename = "240")]
    Fps240,
    #[default]
    #[serde(rename = "unlimited")]
    Unlimited,
}
