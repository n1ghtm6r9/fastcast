use crate::converter::{Converter, NoisePainter, NoiseParameters, draw_plane};
use crate::gpu::GpuDevice;
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D11::{ID3D11DepthStencilView, ID3D11InputLayout};

impl NoisePainter {
    pub fn paint(&self, device: &GpuDevice, converter: &Converter, frame: u32) {
        let parameters = NoiseParameters::new(converter.depth, frame);
        let context = device.context();

        unsafe {
            context.UpdateSubresource(
                &self.constants,
                0,
                None,
                std::ptr::from_ref(&parameters).cast(),
                0,
                0,
            );
            context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            context.IASetInputLayout(None::<&ID3D11InputLayout>);
            context.VSSetShader(&converter.vertex_shader, None);
            context.RSSetState(&converter.rasterizer);
            context.PSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
        }

        draw_plane(
            context,
            &converter.luma_target,
            &self.shader,
            converter.width,
            converter.height,
        );
        draw_plane(
            context,
            &converter.chroma_target,
            &self.shader,
            converter.width / 2,
            converter.height / 2,
        );

        unsafe { context.OMSetRenderTargets(None, None::<&ID3D11DepthStencilView>) };
    }
}
