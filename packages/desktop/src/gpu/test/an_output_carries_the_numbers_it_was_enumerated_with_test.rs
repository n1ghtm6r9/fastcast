use crate::gpu::output_info_of;
use windows::Win32::Graphics::Dxgi::{DXGI_ADAPTER_DESC1, DXGI_OUTPUT_DESC};

#[test]
fn an_output_carries_the_numbers_it_was_enumerated_with() {
    let info = output_info_of(
        1,
        2,
        &DXGI_ADAPTER_DESC1::default(),
        &DXGI_OUTPUT_DESC::default(),
    );

    assert_eq!((info.adapter_index, info.output_index), (1, 2));
}
