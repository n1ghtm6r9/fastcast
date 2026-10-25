use crate::gpu::DesktopError;

#[test]
fn an_odd_size_error_names_both_sides() {
    let error = DesktopError::OddSize {
        width: 63,
        height: 32,
    };

    assert!(error.to_string().contains("63x32"));
}
