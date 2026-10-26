#[cfg(unix)]
pub(crate) const RECEIVE_BUFFER_OPTION: libc::c_int = libc::SO_RCVBUF;

#[cfg(windows)]
pub(crate) const RECEIVE_BUFFER_OPTION: libc::c_int = 0x1002;
