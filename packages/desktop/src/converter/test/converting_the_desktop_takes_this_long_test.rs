use super::{event_query, wait_for_gpu};
use crate::colour::BitDepth;
use crate::converter::Converter;
use crate::duplication::Duplication;
use crate::duplication::test::acquire_desktop_image;
use crate::gpu::GpuDevice;
use crate::gpu::test::first_attached_output;
use std::time::Instant;

#[test]
#[ignore = "benchmark"]
fn converting_the_desktop_takes_this_long() {
    let output = first_attached_output();
    let device = GpuDevice::new(output.adapter_luid).unwrap();
    let mut duplication = Duplication::new(&device, &output).unwrap();
    acquire_desktop_image(&device, &mut duplication);
    let mut converter =
        Converter::new(&device, output.width, output.height, BitDepth::Eight).unwrap();
    let query = event_query(&device);
    let runs = 200;

    let started = Instant::now();
    for _ in 0..runs {
        converter
            .convert(&device, duplication.frame(), duplication.pointer())
            .unwrap();
        wait_for_gpu(&device, &query);
    }
    let mean = started.elapsed().as_secs_f64() * 1000.0 / f64::from(runs);

    println!(
        "converting {}x{} to NV12 took {:.3} ms on average over {} runs",
        output.width, output.height, mean, runs
    );
}
