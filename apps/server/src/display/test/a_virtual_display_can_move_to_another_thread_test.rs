use crate::display::{DisplayError, VirtualDisplay};

fn must_be_send<T: Send>() {}

#[test]
fn a_virtual_display_can_move_to_another_thread() {
    must_be_send::<VirtualDisplay>();
    must_be_send::<DisplayError>();
}
