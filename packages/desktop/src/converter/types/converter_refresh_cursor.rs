use crate::converter::{Converter, upload_cursor};
use crate::duplication::PointerState;
use crate::gpu::{DesktopError, GpuDevice};

impl Converter {
    pub(crate) fn refresh_cursor(
        &mut self,
        device: &GpuDevice,
        pointer: &PointerState,
    ) -> Result<(), DesktopError> {
        let Some(shape) = pointer.shape.as_ref() else {
            self.cursor = None;
            return Ok(());
        };

        let current = self
            .cursor
            .as_ref()
            .is_some_and(|cursor| cursor.generation == pointer.shape_generation);

        if !current {
            self.cursor = upload_cursor(device, shape, pointer.shape_generation)?;
        }

        Ok(())
    }
}
