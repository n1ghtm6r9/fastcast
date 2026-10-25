use crate::display::DisplayPlacement;

impl DisplayPlacement {
    pub fn is_primary(&self) -> bool {
        self.bounds.x == 0 && self.bounds.y == 0
    }
}
