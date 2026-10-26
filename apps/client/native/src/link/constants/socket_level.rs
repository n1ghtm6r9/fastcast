#[cfg(unix)]
pub(crate) const SOCKET_LEVEL: libc::c_int = libc::SOL_SOCKET;

#[cfg(windows)]
pub(crate) const SOCKET_LEVEL: libc::c_int = 0xffff;
