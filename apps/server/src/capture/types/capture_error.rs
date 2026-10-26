use crate::capture::{FFMPEG_INSTALL_HINT, NativeError};
use crate::settings::VideoEncoder;
use media::FlvError;
use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum CaptureError {
    FfmpegNotFound { path: String },
    Spawn { path: String, source: io::Error },
    MissingPipe,
    Demux(FlvError),
    NoEncoderAvailable { tried: Vec<VideoEncoder> },
    EncoderNotNative { encoder: VideoEncoder },
    Native { frames: u64, cause: NativeError },
}

impl fmt::Display for CaptureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FfmpegNotFound { path } => write!(
                formatter,
                "ffmpeg was not found at \"{path}\": install it with `{FFMPEG_INSTALL_HINT}` or put the full path of ffmpeg.exe into «Путь к ffmpeg»"
            ),
            Self::Spawn { path, source } => {
                write!(formatter, "could not start \"{path}\": {source}")
            }
            Self::MissingPipe => write!(formatter, "ffmpeg started without its output pipes"),
            Self::Demux(error) => write!(
                formatter,
                "ffmpeg wrote a stream the server cannot read: {error}"
            ),
            Self::NoEncoderAvailable { tried } => {
                let names: Vec<String> = tried.iter().map(ToString::to_string).collect();

                write!(
                    formatter,
                    "no usable video encoder (tried {})",
                    names.join(", ")
                )
            }
            Self::EncoderNotNative { encoder } => write!(
                formatter,
                "the encoder is set to {encoder}, but native capture encodes with NVENC only"
            ),
            Self::Native { frames, cause } => write!(
                formatter,
                "native capture failed after {frames} frames: {cause}"
            ),
        }
    }
}

impl Error for CaptureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Spawn { source, .. } => Some(source),
            Self::Demux(error) => Some(error),
            Self::Native { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

impl From<FlvError> for CaptureError {
    fn from(error: FlvError) -> Self {
        Self::Demux(error)
    }
}
