use windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use windows::core::{PCWSTR, w};

pub const PATH_REGISTRY_KEYS: [(HKEY, PCWSTR); 2] = [
    (HKEY_CURRENT_USER, w!("Environment")),
    (
        HKEY_LOCAL_MACHINE,
        w!("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"),
    ),
];
