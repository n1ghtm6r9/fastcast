use windows::Win32::Graphics::Gdi::{
    CDS_NORESET, CDS_TYPE, CDS_UPDATEREGISTRY, ChangeDisplaySettingsExW, DEVMODEW, DISP_CHANGE,
    DISP_CHANGE_SUCCESSFUL,
};
use windows::core::HSTRING;

pub fn stage_display_settings(
    device_name: &str,
    settings: &DEVMODEW,
    flags: CDS_TYPE,
) -> Result<(), DISP_CHANGE> {
    let staged = unsafe {
        ChangeDisplaySettingsExW(
            &HSTRING::from(device_name),
            Some(settings),
            None,
            CDS_UPDATEREGISTRY | CDS_NORESET | flags,
            None,
        )
    };

    if staged == DISP_CHANGE_SUCCESSFUL {
        Ok(())
    } else {
        Err(staged)
    }
}
