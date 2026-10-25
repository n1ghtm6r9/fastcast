use crate::colour::BitDepth;
use crate::converter::Converter;

impl Converter {
    pub fn depth(&self) -> BitDepth {
        self.depth
    }
}
