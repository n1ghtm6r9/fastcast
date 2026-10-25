use crate::cursor::MASK_KEEPS_SCREEN;

pub fn apply_masked_pixel(screen: &mut [u8], cursor: &[u8]) {
    let kept = if cursor[3] == MASK_KEEPS_SCREEN {
        u8::MAX
    } else {
        0
    };

    for (screen_channel, cursor_channel) in screen.iter_mut().zip(cursor).take(3) {
        *screen_channel = (*screen_channel & kept) ^ cursor_channel;
    }
}
