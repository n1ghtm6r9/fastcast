use crate::display::DisplayPlacement;

pub fn layout_is_back(saved: &[DisplayPlacement], current: &[DisplayPlacement]) -> bool {
    saved.iter().all(|placement| current.contains(placement))
}
