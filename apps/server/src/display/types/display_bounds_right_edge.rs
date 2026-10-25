use crate::display::DisplayBounds;

impl DisplayBounds {
    pub fn right_edge(&self) -> i32 {
        self.x
            .saturating_add(i32::try_from(self.width).unwrap_or(i32::MAX))
    }
}
