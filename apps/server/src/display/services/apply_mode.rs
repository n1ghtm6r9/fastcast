use crate::display::{DisplayError, DisplayMode};
use windows::Win32::Foundation::POINTL;
use windows::Win32::Graphics::Gdi::{
    CDS_NORESET, CDS_TYPE, CDS_UPDATEREGISTRY, ChangeDisplaySettingsExW, DEVMODEW, DEVMODEW_0,
    DEVMODEW_0_1, DISP_CHANGE, DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT,
    DM_PELSWIDTH, DM_POSITION,
};
use windows::core::{HSTRING, PCWSTR};

pub fn apply_mode(
    device_name: &str,
    mode: DisplayMode,
    x: i32,
    y: i32,
) -> Result<(), DisplayError> {
    let refused = |result: DISP_CHANGE| DisplayError::ModeChangeRefused {
        device: device_name.to_owned(),
        mode,
        code: result.0,
    };
    let settings = DEVMODEW {
        dmSize: size_of::<DEVMODEW>() as u16,
        dmFields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY,
        Anonymous1: DEVMODEW_0 {
            Anonymous2: DEVMODEW_0_1 {
                dmPosition: POINTL { x, y },
                ..Default::default()
            },
        },
        dmPelsWidth: mode.width,
        dmPelsHeight: mode.height,
        dmDisplayFrequency: mode.refresh_hz,
        ..Default::default()
    };
    let device_name = HSTRING::from(device_name);

    let staged = unsafe {
        ChangeDisplaySettingsExW(
            &device_name,
            Some(&settings),
            None,
            CDS_UPDATEREGISTRY | CDS_NORESET,
            None,
        )
    };

    if staged != DISP_CHANGE_SUCCESSFUL {
        return Err(refused(staged));
    }

    let applied =
        unsafe { ChangeDisplaySettingsExW(PCWSTR::null(), None, None, CDS_TYPE(0), None) };

    if applied != DISP_CHANGE_SUCCESSFUL {
        return Err(refused(applied));
    }

    Ok(())
}
