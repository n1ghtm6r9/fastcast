use windows::Win32::Graphics::Gdi::{DEVMODEW, ENUM_DISPLAY_SETTINGS_MODE, EnumDisplaySettingsW};
use windows::core::HSTRING;

pub fn display_settings_of(
    device_name: &HSTRING,
    mode_number: ENUM_DISPLAY_SETTINGS_MODE,
) -> Option<DEVMODEW> {
    let mut settings = DEVMODEW {
        dmSize: size_of::<DEVMODEW>() as u16,
        ..Default::default()
    };

    unsafe { EnumDisplaySettingsW(device_name, mode_number, &mut settings) }
        .as_bool()
        .then_some(settings)
}
