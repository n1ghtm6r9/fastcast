use super::bgra_texture;
use crate::colour::{BitDepth, Planes};
use crate::converter::{Converter, read_planes};
use crate::duplication::PointerState;
use crate::gpu::GpuDevice;

pub fn gpu_planes(
    device: &GpuDevice,
    bgra: &[u8],
    width: u32,
    height: u32,
    depth: BitDepth,
    pointer: &PointerState,
) -> Planes {
    let source = bgra_texture(device, width, height, bgra);
    let mut converter = Converter::new(device, width, height, depth).unwrap();

    converter.convert(device, &source, pointer).unwrap();
    read_planes(device, converter.output(), depth).unwrap()
}
