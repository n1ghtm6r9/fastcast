use windows::Win32::Graphics::Gdi::{DEVMODEW, DM_PELSHEIGHT, DM_PELSWIDTH, DM_POSITION};

pub fn detached_settings() -> DEVMODEW {
    DEVMODEW {
        dmSize: size_of::<DEVMODEW>() as u16,
        dmFields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT,
        ..Default::default()
    }
}
