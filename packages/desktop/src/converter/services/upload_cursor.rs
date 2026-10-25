use crate::converter::{UploadedCursor, create_shader_view, cursor_is_complete};
use crate::cursor::{BYTES_PER_PIXEL, CursorImage};
use crate::duplication::FRAME_FORMAT;
use crate::gpu::{DesktopError, GpuDevice, create_texture};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_BIND_SHADER_RESOURCE, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_IMMUTABLE,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;

pub fn upload_cursor(
    device: &GpuDevice,
    shape: &CursorImage,
    generation: u64,
) -> Result<Option<UploadedCursor>, DesktopError> {
    if !cursor_is_complete(shape) {
        return Ok(None);
    }

    let description = D3D11_TEXTURE2D_DESC {
        Width: shape.width,
        Height: shape.height,
        MipLevels: 1,
        ArraySize: 1,
        Format: FRAME_FORMAT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_IMMUTABLE,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };
    let initial_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: shape.pixels.as_ptr().cast(),
        SysMemPitch: shape.width * BYTES_PER_PIXEL as u32,
        SysMemSlicePitch: 0,
    };
    let texture = create_texture(device, &description, Some(&initial_data))?;

    Ok(Some(UploadedCursor {
        generation,
        width: shape.width as i32,
        height: shape.height as i32,
        kind: shape.kind,
        view: create_shader_view(device, &texture)?,
    }))
}
