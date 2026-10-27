use super::FakeScaler;
use crate::display::{DisplayError, ScaleLevels, ScreenScaler};
use std::io;

impl ScreenScaler for FakeScaler {
    fn levels(&self, output: u32) -> Result<ScaleLevels, DisplayError> {
        self.levels
            .lock()
            .expect("the levels lock is not poisoned")
            .filter(|_| output == self.output)
            .ok_or(DisplayError::NoSuchOutput { output })
    }

    fn set_level(&self, output: u32, level: i32) -> Result<(), DisplayError> {
        self.changes
            .lock()
            .expect("the changes lock is not poisoned")
            .push((output, level));

        if self.refuses {
            return Err(DisplayError::System {
                call: "DisplayConfigSetDeviceInfo",
                source: io::Error::from_raw_os_error(5),
            });
        }

        if let Some(levels) = self
            .levels
            .lock()
            .expect("the levels lock is not poisoned")
            .as_mut()
        {
            levels.current = level;
        }

        Ok(())
    }
}
