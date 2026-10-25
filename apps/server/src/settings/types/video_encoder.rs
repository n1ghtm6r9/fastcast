use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoEncoder {
    Nvenc,
    Amf,
    Qsv,
    X264,
}

impl fmt::Display for VideoEncoder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            VideoEncoder::Nvenc => "nvenc",
            VideoEncoder::Amf => "amf",
            VideoEncoder::Qsv => "qsv",
            VideoEncoder::X264 => "x264",
        };

        write!(formatter, "{}", name)
    }
}
