#[cfg(windows)]
use crate::display::DesktopTakeover;
use crate::display::DisplayMode;

pub struct VirtualDisplay {
    pub(crate) output_index: u32,
    pub(crate) mode: DisplayMode,
    pub(crate) device_name: String,
    #[cfg(windows)]
    pub(crate) _takeover: DesktopTakeover,
}
