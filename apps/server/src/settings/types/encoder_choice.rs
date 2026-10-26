use crate::settings::{UnknownEncoderName, VideoEncoder, encoder_choice_named};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub enum EncoderChoice {
    Auto,
    Fixed(VideoEncoder),
}

impl fmt::Display for EncoderChoice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncoderChoice::Auto => write!(formatter, "auto"),
            EncoderChoice::Fixed(encoder) => write!(formatter, "{}", encoder),
        }
    }
}

impl From<EncoderChoice> for String {
    fn from(choice: EncoderChoice) -> Self {
        choice.to_string()
    }
}

impl TryFrom<String> for EncoderChoice {
    type Error = UnknownEncoderName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        encoder_choice_named(&name).ok_or(UnknownEncoderName(name))
    }
}
