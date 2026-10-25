use crate::display::DisplayPlacement;
use windows::Win32::Graphics::Gdi::DEVMODEW;

pub struct SavedDisplay {
    pub(crate) placement: DisplayPlacement,
    pub(crate) settings: DEVMODEW,
}
