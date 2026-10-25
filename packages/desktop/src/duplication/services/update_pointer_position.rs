use crate::duplication::PointerState;
use windows::Win32::Graphics::Dxgi::DXGI_OUTDUPL_FRAME_INFO;

pub fn update_pointer_position(pointer: &mut PointerState, info: &DXGI_OUTDUPL_FRAME_INFO) -> bool {
    if info.LastMouseUpdateTime == 0 {
        return false;
    }

    pointer.visible = info.PointerPosition.Visible.as_bool();
    pointer.left = info.PointerPosition.Position.x;
    pointer.top = info.PointerPosition.Position.y;
    true
}
