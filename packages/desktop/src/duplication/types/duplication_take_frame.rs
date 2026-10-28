use crate::duplication::{Duplication, FrameUpdate, update_pointer_position};
use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Dxgi::{DXGI_OUTDUPL_FRAME_INFO, IDXGIResource};
use windows::core::Interface;

impl Duplication {
    pub(crate) fn take_frame(
        &mut self,
        device: &GpuDevice,
        info: &DXGI_OUTDUPL_FRAME_INFO,
        resource: Option<&IDXGIResource>,
    ) -> Result<FrameUpdate, DesktopError> {
        let desktop = info.LastPresentTime != 0;

        if desktop {
            let resource = resource
                .ok_or_else(|| missing_interface("IDXGIOutputDuplication::AcquireNextFrame"))?;
            let image: ID3D11Texture2D = resource
                .cast()
                .map_err(api_error("IDXGIResource::QueryInterface"))?;

            unsafe { device.context().CopyResource(&self.frame, &image) };
        }

        let moved = update_pointer_position(&mut self.pointer, info);
        let reshaped = self.take_pointer_shape(info.PointerShapeBufferSize)?;

        Ok(FrameUpdate {
            desktop,
            pointer: moved || reshaped,
            presented: info.LastPresentTime,
        })
    }
}
