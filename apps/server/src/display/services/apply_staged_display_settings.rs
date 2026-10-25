use windows::Win32::Graphics::Gdi::{
    CDS_TYPE, ChangeDisplaySettingsExW, DISP_CHANGE, DISP_CHANGE_SUCCESSFUL,
};
use windows::core::PCWSTR;

pub fn apply_staged_display_settings() -> Result<(), DISP_CHANGE> {
    let applied =
        unsafe { ChangeDisplaySettingsExW(PCWSTR::null(), None, None, CDS_TYPE(0), None) };

    if applied == DISP_CHANGE_SUCCESSFUL {
        Ok(())
    } else {
        Err(applied)
    }
}
