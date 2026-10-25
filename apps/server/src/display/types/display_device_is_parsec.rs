use crate::display::{DisplayDevice, PARSEC_ADAPTER_MARKER};

impl DisplayDevice {
    pub fn is_parsec(&self) -> bool {
        self.adapter
            .to_ascii_lowercase()
            .contains(&PARSEC_ADAPTER_MARKER.to_ascii_lowercase())
    }
}
