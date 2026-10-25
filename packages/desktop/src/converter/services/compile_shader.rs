use crate::converter::{SHADER_COMPILE_FLAGS, SHADER_ENTRY_POINT, blob_bytes, compiler_message};
use crate::gpu::{DesktopError, missing_interface};
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
use windows::Win32::Graphics::Direct3D::{ID3DBlob, ID3DInclude};
use windows::core::PCSTR;

pub fn compile_shader(source: &str, target: PCSTR) -> Result<Vec<u8>, DesktopError> {
    let mut code: Option<ID3DBlob> = None;
    let mut messages: Option<ID3DBlob> = None;

    let result = unsafe {
        D3DCompile(
            source.as_ptr().cast(),
            source.len(),
            PCSTR::null(),
            None,
            None::<&ID3DInclude>,
            SHADER_ENTRY_POINT,
            target,
            SHADER_COMPILE_FLAGS,
            0,
            &mut code,
            Some(&mut messages),
        )
    };

    match result {
        Ok(()) => code
            .as_ref()
            .map(blob_bytes)
            .ok_or_else(|| missing_interface("D3DCompile")),
        Err(error) => Err(DesktopError::ShaderCompile(compiler_message(
            messages, &error,
        ))),
    }
}
