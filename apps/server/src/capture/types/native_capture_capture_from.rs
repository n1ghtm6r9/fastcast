use crate::capture::{NativeCapture, NativeError, shown_at, spin_until};
use desktop::{DesktopError, Duplication};
use std::time::Instant;
use tracing::info;

impl NativeCapture<'_> {
    pub(crate) fn capture_from(&mut self, mut duplication: Duplication) -> Result<(), NativeError> {
        let asked = Instant::now();
        let timeout = self.clock.until_repeat(asked);
        let acquired = duplication.acquire(&self.device, timeout);
        let arrived = Instant::now();

        match acquired {
            Ok(Some(update)) => {
                if self.pending_arrived.is_some() {
                    self.stats.replaced();
                }

                self.picture.arrived();
                self.pending_arrived = Some(arrived);
                self.pending_shown =
                    shown_at(update.presented, Instant::now()).or(self.pending_shown);

                if self.clock.may_encode(arrived) {
                    self.convert_waiting(&duplication)?;
                    self.encode_and_publish(arrived)?;
                }

                self.duplication = Some(duplication);

                Ok(())
            }
            Ok(None) => {
                self.stats
                    .waited(arrived.saturating_duration_since(asked + timeout));
                self.convert_waiting(&duplication)?;
                self.duplication = Some(duplication);

                if self.picture.exists() {
                    spin_until(arrived + self.clock.until_repeat(arrived));
                }

                self.repeat(Instant::now())
            }
            Err(DesktopError::AccessLost) => {
                info!("access to the desktop was lost, opening the duplication again");

                self.convert_waiting(&duplication)
            }
            Err(error) => Err(NativeError::Desktop(error)),
        }
    }
}
