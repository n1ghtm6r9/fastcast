use crate::display::DisplaySource;
use windows::Win32::Devices::Display::{
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE,
};

pub fn device_info_header<T>(
    source: DisplaySource,
    kind: DISPLAYCONFIG_DEVICE_INFO_TYPE,
) -> DISPLAYCONFIG_DEVICE_INFO_HEADER {
    DISPLAYCONFIG_DEVICE_INFO_HEADER {
        r#type: kind,
        size: u32::try_from(size_of::<T>()).expect("a display config packet is a few dozen bytes"),
        adapterId: source.adapter,
        id: source.id,
    }
}
