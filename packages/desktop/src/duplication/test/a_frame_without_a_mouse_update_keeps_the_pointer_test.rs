use crate::duplication::{PointerState, update_pointer_position};
use windows::Win32::Foundation::POINT;
use windows::Win32::Graphics::Dxgi::{DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTDUPL_POINTER_POSITION};
use windows::core::BOOL;

#[test]
fn a_frame_without_a_mouse_update_keeps_the_pointer() {
    let mut pointer = PointerState {
        visible: true,
        left: 7,
        top: 9,
        ..PointerState::default()
    };
    let info = DXGI_OUTDUPL_FRAME_INFO {
        LastMouseUpdateTime: 0,
        PointerPosition: DXGI_OUTDUPL_POINTER_POSITION {
            Position: POINT { x: 100, y: 200 },
            Visible: BOOL::from(false),
        },
        ..DXGI_OUTDUPL_FRAME_INFO::default()
    };

    let moved = update_pointer_position(&mut pointer, &info);

    assert!(!moved);
    assert_eq!((pointer.visible, pointer.left, pointer.top), (true, 7, 9));
}
