use crate::capture::{FrameClock, KeyFrameGate, Picture, ReopenFailures};
use crate::hub::Hub;
use desktop::{Converter, Duplication, GpuDevice, OutputInfo};
use media::VideoCodec;
use nvenc::Encoder;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

pub struct NativeCapture<'a> {
    pub(crate) encoder: Encoder,
    pub(crate) converter: Converter,
    pub(crate) duplication: Option<Duplication>,
    pub(crate) device: GpuDevice,
    pub(crate) hub: &'a Hub,
    pub(crate) stop: &'a AtomicBool,
    pub(crate) codec: VideoCodec,
    pub(crate) fps: u32,
    pub(crate) output: OutputInfo,
    pub(crate) size: (u32, u32),
    pub(crate) clock: FrameClock,
    pub(crate) key_frames: KeyFrameGate,
    pub(crate) reopen_failures: ReopenFailures,
    pub(crate) started: Instant,
    pub(crate) picture: Picture,
    pub(crate) frames: u64,
    pub(crate) pending_shown: Option<Instant>,
    pub(crate) pending_arrived: Option<Instant>,
}
