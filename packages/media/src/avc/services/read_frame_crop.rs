use crate::avc::{BitReader, FrameCrop};

pub fn read_frame_crop(reader: &mut BitReader) -> Option<FrameCrop> {
    let cropping_present = reader.read_bit()? == 1;

    if !cropping_present {
        return Some(FrameCrop::default());
    }

    Some(FrameCrop {
        left: reader.read_ue()?,
        right: reader.read_ue()?,
        top: reader.read_ue()?,
        bottom: reader.read_ue()?,
    })
}
