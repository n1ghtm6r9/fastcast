use crate::duplication::FRAME_FORMAT;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

pub const DUPLICATION_FORMATS: [DXGI_FORMAT; 1] = [FRAME_FORMAT];
