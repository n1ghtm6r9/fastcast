use crate::cursor::ALPHA_MAX;

pub fn blend_alpha_pixel(screen: &mut [u8], cursor: &[u8]) {
    let alpha = u32::from(cursor[3]);

    for (screen_channel, cursor_channel) in screen.iter_mut().zip(cursor).take(3) {
        let weighted =
            u32::from(*cursor_channel) * alpha + u32::from(*screen_channel) * (ALPHA_MAX - alpha);

        *screen_channel = ((weighted + ALPHA_MAX / 2) / ALPHA_MAX) as u8;
    }
}
