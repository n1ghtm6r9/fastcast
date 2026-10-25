use crate::capture::{LONGEST_ENVIRONMENT_VALUE, PATH_REGISTRY_VALUE};
use crate::display::text_of_wide;
use windows::Win32::System::Registry::{HKEY, RRF_RT_REG_SZ, RegGetValueW};
use windows::core::PCWSTR;

pub fn saved_path_of(root: HKEY, key: PCWSTR) -> Option<String> {
    let mut text = vec![0u16; LONGEST_ENVIRONMENT_VALUE];
    let mut size = u32::try_from(size_of_val(text.as_slice())).ok()?;
    let read = unsafe {
        RegGetValueW(
            root,
            key,
            PATH_REGISTRY_VALUE,
            RRF_RT_REG_SZ,
            None,
            Some(text.as_mut_ptr().cast()),
            Some(&mut size),
        )
    };

    read.is_ok().then(|| text_of_wide(&text))
}
