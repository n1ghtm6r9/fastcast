use windows::Win32::Foundation::LUID;

#[derive(Debug, Clone, Copy)]
pub struct DisplaySource {
    pub adapter: LUID,
    pub id: u32,
}
