#[cfg(windows)]
use crate::app::WINDOW_TITLE;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
#[cfg(windows)]
use windows::core::HSTRING;

#[cfg(windows)]
pub fn show_problem(text: &str) {
    unsafe {
        MessageBoxW(
            None,
            &HSTRING::from(text),
            &HSTRING::from(WINDOW_TITLE),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(windows))]
pub fn show_problem(text: &str) {
    eprintln!("{text}");
}
