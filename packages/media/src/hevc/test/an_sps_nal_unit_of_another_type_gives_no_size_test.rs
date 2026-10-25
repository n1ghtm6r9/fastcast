use super::HevcSpsParts;
use crate::hevc::video_size_from_hevc_sps;

#[test]
fn an_sps_nal_unit_of_another_type_gives_no_size() {
    let mut nal_unit = HevcSpsParts::default().nal_unit();

    nal_unit[0] = 0x44;

    assert_eq!(video_size_from_hevc_sps(&nal_unit), None);
}
