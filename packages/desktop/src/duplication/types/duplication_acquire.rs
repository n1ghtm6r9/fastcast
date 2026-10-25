use crate::duplication::{Duplication, FrameUpdate, duplication_error, timeout_milliseconds};
use crate::gpu::{DesktopError, GpuDevice};
use std::time::Duration;
use windows::Win32::Graphics::Dxgi::{DXGI_ERROR_WAIT_TIMEOUT, DXGI_OUTDUPL_FRAME_INFO};

impl Duplication {
    pub fn acquire(
        &mut self,
        device: &GpuDevice,
        timeout: Duration,
    ) -> Result<Option<FrameUpdate>, DesktopError> {
        let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut resource = None;

        let acquired = unsafe {
            self.duplication.AcquireNextFrame(
                timeout_milliseconds(timeout),
                &mut info,
                &mut resource,
            )
        };

        match acquired {
            Ok(()) => {}
            Err(error) if error.code() == DXGI_ERROR_WAIT_TIMEOUT => return Ok(None),
            Err(error) => {
                return Err(duplication_error(
                    "IDXGIOutputDuplication::AcquireNextFrame",
                    error,
                ));
            }
        }

        let update = self.take_frame(device, &info, resource.as_ref());
        let released = unsafe { self.duplication.ReleaseFrame() };

        let update = update?;
        released
            .map_err(|error| duplication_error("IDXGIOutputDuplication::ReleaseFrame", error))?;

        Ok(Some(update))
    }
}
