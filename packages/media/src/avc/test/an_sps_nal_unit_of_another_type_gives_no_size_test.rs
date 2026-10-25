use super::SpsParts;
use crate::avc::video_size_from_sps;

#[test]
fn an_sps_nal_unit_of_another_type_gives_no_size() {
    let mut nal_unit = SpsParts::default().nal_unit();

    nal_unit[0] = 0x68;

    assert_eq!(video_size_from_sps(&nal_unit), None);
}
