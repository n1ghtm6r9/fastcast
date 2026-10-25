#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FcResult {
    pub status: i32,
    pub call: u32,
}
