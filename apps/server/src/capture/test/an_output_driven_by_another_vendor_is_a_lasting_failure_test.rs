use crate::capture::NativeError;

#[test]
fn an_output_driven_by_another_vendor_is_a_lasting_failure() {
    let error = NativeError::NotNvidia {
        adapter: "a test adapter".to_owned(),
    };

    assert!(error.is_lasting());
}
