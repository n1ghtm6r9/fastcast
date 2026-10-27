use crate::display::DisplayError;

#[test]
fn an_output_that_does_not_exist_is_named_by_its_number() {
    assert_eq!(
        DisplayError::NoSuchOutput { output: 3 }.to_string(),
        "output 3 of the first DXGI adapter does not exist"
    );
}
