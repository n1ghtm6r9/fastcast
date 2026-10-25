#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FrameCrop {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}
