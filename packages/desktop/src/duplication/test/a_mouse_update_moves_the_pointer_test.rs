use crate::duplication::{PointerState, update_pointer_position};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Dxgi::{DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTDUPL_POINTER_POSITION};
use windows::core::BOOL;

#[test]
fn a_mouse_update_moves_the_pointer() {
    let mut pointer = PointerState::default();
    let info = DXGI_OUTDUPL_FRAME_INFO {
        LastMouseUpdateTime: 5,
        PointerPosition: DXGI_OUTDUPL_POINTER_POSITION {
            Position: POINT { x: 10, y: -4 },
            Visible: BOOL::from(true),
        },
        ..DXGI_OUTDUPL_FRAME_INFO::default()
    };

    let moved = update_pointer_position(&mut pointer, &info);

    assert!(moved);
    assert_eq!((pointer.visible, pointer.left, pointer.top), (true, 10, -4));
}
