use crate::duplication::{Duplication, read_pointer_shape};
use crate::gpu::DesktopError;

impl Duplication {
    pub(crate) fn take_pointer_shape(&mut self, buffer_size: u32) -> Result<bool, DesktopError> {
        if buffer_size == 0 {
            return Ok(false);
        }

        self.pointer.shape = read_pointer_shape(&self.duplication, buffer_size)?;
        self.pointer.shape_generation += 1;

        Ok(true)
    }
}
