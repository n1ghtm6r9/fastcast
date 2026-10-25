use crate::converter::blob_bytes;
use windows::Win32::Graphics::Direct3D::ID3DBlob;

pub fn compiler_message(messages: Option<ID3DBlob>, error: &windows::core::Error) -> String {
    match messages {
        Some(blob) => String::from_utf8_lossy(&blob_bytes(&blob))
            .trim_end_matches(['\0', '\r', '\n', ' '])
            .to_string(),
        None => error.to_string(),
    }
}
