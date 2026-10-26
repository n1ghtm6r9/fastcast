use crate::capture::NativeError;

#[test]
fn an_output_of_another_vendor_names_its_adapter() {
    let error = NativeError::NotNvidia {
        adapter: "Intel(R) UHD Graphics".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        "the output is driven by Intel(R) UHD Graphics, not by an NVIDIA GPU"
    );
}
