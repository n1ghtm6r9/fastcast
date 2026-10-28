mod colour;
#[cfg(windows)]
mod converter;
mod cursor;
#[cfg(windows)]
mod duplication;
#[cfg(windows)]
mod gpu;

pub use colour::{BitDepth, Planes, YCbCr, reference_planes, ycbcr_of};
#[cfg(windows)]
pub use converter::{Converter, read_planes};
pub use cursor::{
    CursorImage, CursorKind, POINTER_SHAPE_COLOR, POINTER_SHAPE_MASKED_COLOR,
    POINTER_SHAPE_MONOCHROME, composite_cursor, decode_cursor_shape,
};
#[cfg(windows)]
pub use duplication::{Duplication, FrameUpdate, PointerState, present_age};
#[cfg(windows)]
pub use gpu::{AdapterLuid, DesktopError, GpuDevice, NVIDIA_VENDOR_ID, OutputInfo, list_outputs};
