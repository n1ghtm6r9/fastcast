#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOfDay {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub millisecond: u16,
}
