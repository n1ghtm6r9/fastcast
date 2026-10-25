use crate::settings::VideoEncoder;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
