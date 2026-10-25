use crate::converter::{Converter, ShaderParameters, create_shader_view, draw_plane};
use crate::duplication::PointerState;
use crate::gpu::{DesktopError, GpuDevice};
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11DepthStencilView, ID3D11InputLayout, ID3D11Texture2D,
};

impl Converter {
    pub fn convert(
        &mut self,
        device: &GpuDevice,
        source: &ID3D11Texture2D,
        pointer: &PointerState,
    ) -> Result<(), DesktopError> {
        self.refresh_cursor(device, pointer)?;

        let source_view = create_shader_view(device, source)?;
        let cursor_view = self.cursor.as_ref().map(|cursor| cursor.view.clone());
        let parameters =
            ShaderParameters::new(self.depth, self.width, pointer, self.cursor.as_ref());
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
            context.VSSetShader(&self.vertex_shader, None);
            context.RSSetState(&self.rasterizer);
            context.PSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            context.PSSetShaderResources(0, Some(&[Some(source_view), cursor_view]));
        }

        draw_plane(
            context,
            &self.luma_target,
            &self.luma_shader,
            self.width,
            self.height,
        );
        draw_plane(
            context,
            &self.chroma_target,
            &self.chroma_shader,
            self.width / 2,
            self.height / 2,
        );

        unsafe {
            context.OMSetRenderTargets(None, None::<&ID3D11DepthStencilView>);
            context.PSSetShaderResources(0, Some(&[None, None]));
        }

        Ok(())
    }
}
