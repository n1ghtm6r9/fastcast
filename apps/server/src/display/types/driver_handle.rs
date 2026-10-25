use windows::Win32::Foundation::HANDLE;
use windows::core::Owned;

pub struct DriverHandle {
    pub(crate) handle: Owned<HANDLE>,
}

unsafe impl Send for DriverHandle {}
unsafe impl Sync for DriverHandle {}
