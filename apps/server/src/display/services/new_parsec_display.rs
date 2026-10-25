use crate::display::DisplayDevice;

pub fn new_parsec_display<'a>(
    attached: &'a [DisplayDevice],
    before: &[DisplayDevice],
) -> Option<&'a DisplayDevice> {
    attached
        .iter()
        .find(|device| device.is_parsec() && !before.iter().any(|known| known.name == device.name))
}
