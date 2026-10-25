use crate::duplication::create_frame_texture;
use crate::gpu::GpuDevice;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;

pub fn bgra_texture(device: &GpuDevice, width: u32, height: u32, bytes: &[u8]) -> ID3D11Texture2D {
    assert_eq!(bytes.len(), width as usize * height as usize * 4);

    let texture = create_frame_texture(device, width, height).unwrap();

    unsafe {
        device
            .context()
            .UpdateSubresource(&texture, 0, None, bytes.as_ptr().cast(), width * 4, 0)
    };

    texture
}
