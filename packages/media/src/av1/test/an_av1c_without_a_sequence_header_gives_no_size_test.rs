use super::{config_obus_of, obu_of, real_av1c_record_named};
use crate::video_size_from_av1c;

#[test]
fn an_av1c_without_a_sequence_header_gives_no_size() {
    let record = real_av1c_record_named("nvenc_main_1920x1080").av1c;
    let bytes_of_a_sequence_header = &config_obus_of(&record)[2..];
    let av1c = [&record[..4], &obu_of(5, bytes_of_a_sequence_header)].concat();

    assert_eq!(video_size_from_av1c(&av1c), None);
}
