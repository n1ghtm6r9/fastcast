#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FcPacket {
    pub data: *const u8,
    pub size: u32,
    pub keyframe: u32,
}
