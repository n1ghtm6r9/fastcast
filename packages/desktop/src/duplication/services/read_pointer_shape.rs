use crate::cursor::{CursorImage, decode_cursor_shape};
use crate::duplication::duplication_error;
use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::{DXGI_OUTDUPL_POINTER_SHAPE_INFO, IDXGIOutputDuplication};

pub fn read_pointer_shape(
    duplication: &IDXGIOutputDuplication,
    buffer_size: u32,
) -> Result<Option<CursorImage>, DesktopError> {
    let mut buffer = vec![0u8; buffer_size as usize];
    let mut required = 0u32;
    let mut shape = DXGI_OUTDUPL_POINTER_SHAPE_INFO::default();

    unsafe {
        duplication.GetFramePointerShape(
            buffer_size,
            buffer.as_mut_ptr().cast(),
            &mut required,
            &mut shape,
        )
    }
    .map_err(|error| duplication_error("IDXGIOutputDuplication::GetFramePointerShape", error))?;

    buffer.truncate(required as usize);

    Ok(decode_cursor_shape(
        shape.Type,
        shape.Width,
        shape.Height,
        shape.Pitch,
        &buffer,
    ))
}
