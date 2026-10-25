use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::core::{IUnknown_Vtbl, Interface};

pub fn reference_count(texture: &ID3D11Texture2D) -> u32 {
    let raw = texture.as_raw();
    let vtable = unsafe { &**(raw as *mut *const IUnknown_Vtbl) };
    let with_probe = unsafe { (vtable.AddRef)(raw) };
    unsafe { (vtable.Release)(raw) };

    with_probe - 1
}
