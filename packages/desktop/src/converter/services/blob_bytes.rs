use windows::Win32::Graphics::Direct3D::ID3DBlob;

pub fn blob_bytes(blob: &ID3DBlob) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize())
    }
    .to_vec()
}
