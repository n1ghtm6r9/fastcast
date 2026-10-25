use crate::display::{DisplayError, DriverHandle, driver_interface_path, system_error};
use windows::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::core::Owned;

impl DriverHandle {
    pub fn open() -> Result<DriverHandle, DisplayError> {
        let path = driver_interface_path()?;
        let access = (GENERIC_READ | GENERIC_WRITE).0;
        let handle = unsafe {
            CreateFileW(
                &path,
                access,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }
        .map_err(system_error("CreateFileW"))?;

        Ok(DriverHandle {
            handle: unsafe { Owned::new(handle) },
        })
    }
}
