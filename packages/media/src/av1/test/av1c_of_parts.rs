use super::Av1SequenceHeaderParts;
use crate::{VideoCodec, split_access_unit};

pub fn av1c_of_parts(parts: &Av1SequenceHeaderParts) -> Option<Vec<u8>> {
    split_access_unit(VideoCodec::Av1, &parts.obu()).record
}
