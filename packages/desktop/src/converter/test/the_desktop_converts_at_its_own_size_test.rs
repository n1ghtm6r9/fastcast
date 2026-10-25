use crate::colour::BitDepth;
use crate::converter::Converter;
use crate::duplication::Duplication;
use crate::duplication::test::acquire_desktop_image;
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;
use windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE2D_DESC;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_NV12;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_desktop_converts_at_its_own_size() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();
    acquire_desktop_image(&device, &mut duplication);
    let mut converter =
        Converter::new(&device, output.width, output.height, BitDepth::Eight).unwrap();

    converter
        .convert(&device, duplication.frame(), duplication.pointer())
        .unwrap();

    let mut description = D3D11_TEXTURE2D_DESC::default();
    unsafe { converter.output().GetDesc(&mut description) };
    assert_eq!(
        (description.Width, description.Height),
        (output.width, output.height)
    );
    assert_eq!(description.Format, DXGI_FORMAT_NV12);
}
