use crate::display::{DisplayError, DriverHandle, IOCTL_INPUT_SIZE};
use windows::Win32::System::IO::DeviceIoControl;

impl DriverHandle {
    pub fn request(&self, code: u32, input: [u8; IOCTL_INPUT_SIZE]) -> Result<u32, DisplayError> {
        let mut answer = 0u32;
        let mut answered_bytes = 0u32;

        unsafe {
            DeviceIoControl(
                *self.handle,
                code,
                Some(input.as_ptr().cast()),
                IOCTL_INPUT_SIZE as u32,
                Some((&raw mut answer).cast()),
                size_of::<u32>() as u32,
                Some(&mut answered_bytes),
                None,
            )
        }
        .map_err(|source| DisplayError::DriverRequest {
            code,
            source: source.into(),
        })?;

        Ok(answer)
    }
}
