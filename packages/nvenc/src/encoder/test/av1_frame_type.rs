use super::obus;

const OBU_FRAME_HEADER: u8 = 3;
const OBU_FRAME: u8 = 6;

pub fn av1_frame_type(temporal_unit: &[u8]) -> u8 {
    let (_, frame_header) = obus(temporal_unit)
        .into_iter()
        .find(|(kind, _)| *kind == OBU_FRAME_HEADER || *kind == OBU_FRAME)
        .expect("a frame header");
    let show_existing_frame = frame_header[0] >> 7;

    assert_eq!(show_existing_frame, 0);
    (frame_header[0] >> 5) & 0b11
}
