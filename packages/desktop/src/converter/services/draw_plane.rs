use crate::converter::FULLSCREEN_TRIANGLE_VERTICES;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_VIEWPORT, ID3D11DepthStencilView, ID3D11DeviceContext, ID3D11PixelShader,
    ID3D11RenderTargetView,
};

pub fn draw_plane(
    context: &ID3D11DeviceContext,
    target: &ID3D11RenderTargetView,
    shader: &ID3D11PixelShader,
    width: u32,
    height: u32,
) {
    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: width as f32,
        Height: height as f32,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };

    unsafe {
        context.OMSetRenderTargets(
            Some(&[Some(target.clone())]),
            None::<&ID3D11DepthStencilView>,
        );
        context.RSSetViewports(Some(&[viewport]));
        context.PSSetShader(shader, None);
        context.Draw(FULLSCREEN_TRIANGLE_VERTICES, 0);
    }
}
