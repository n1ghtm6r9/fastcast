use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownEncoderName(pub String);

impl fmt::Display for UnknownEncoderName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown video encoder \"{}\", expected auto, nvenc, amf, qsv or x264",
            self.0
        )
    }
}
