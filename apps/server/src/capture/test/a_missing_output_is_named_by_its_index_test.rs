use crate::capture::NativeError;

#[test]
fn a_missing_output_is_named_by_its_index() {
    assert_eq!(
        NativeError::OutputMissing { index: 3 }.to_string(),
        "output 3 of the first graphics adapter is not there"
    );
}
