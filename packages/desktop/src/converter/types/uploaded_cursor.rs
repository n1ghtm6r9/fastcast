use crate::cursor::CursorKind;
use windows::Win32::Graphics::Direct3D11::ID3D11ShaderResourceView;

pub struct UploadedCursor {
    pub(crate) generation: u64,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) kind: CursorKind,
    pub(crate) view: ID3D11ShaderResourceView,
}
