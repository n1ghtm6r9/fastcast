use super::acquire_desktop_image;
use crate::duplication::Duplication;
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;
use windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE2D_DESC;

#[test]
#[ignore = "needs a GPU and a desktop that is updating"]
fn the_desktop_image_arrives_in_a_texture_of_the_output_size() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();

    acquire_desktop_image(&device, &mut duplication);

    let mut description = D3D11_TEXTURE2D_DESC::default();
    unsafe { duplication.frame().GetDesc(&mut description) };
    assert_eq!(
        (duplication.width(), duplication.height()),
        (output.width, output.height)
    );
    assert_eq!(
        (description.Width, description.Height),
        (output.width, output.height)
    );
}
