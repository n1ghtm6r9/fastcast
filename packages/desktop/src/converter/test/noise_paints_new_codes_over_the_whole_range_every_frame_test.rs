use super::{distinct_codes, share_changed};
use crate::colour::{BitDepth, Planes};
use crate::converter::{Converter, NoisePainter, read_planes};
use crate::gpu::test::first_output_device;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn noise_paints_new_codes_over_the_whole_range_every_frame() {
    let device = first_output_device();
    let painter = NoisePainter::new(&device).unwrap();

    for (depth, codes) in [(BitDepth::Eight, 256), (BitDepth::Ten, 1024)] {
        let converter = Converter::new(&device, 128, 64, depth).unwrap();

        painter.paint(&device, &converter, 1);
        let first: Planes = read_planes(&device, converter.output(), depth).unwrap();
        painter.paint(&device, &converter, 2);
        let second: Planes = read_planes(&device, converter.output(), depth).unwrap();

        assert!(
            share_changed(&first.luma, &second.luma) > 0.95,
            "{depth:?} luma"
        );
        assert!(
            share_changed(&first.chroma, &second.chroma) > 0.95,
            "{depth:?} chroma"
        );
        assert!(
            distinct_codes(&second.luma) > codes * 3 / 4,
            "{depth:?} luma spread"
        );
        assert!(
            distinct_codes(&second.chroma) > codes / 2,
            "{depth:?} chroma spread"
        );
        assert!(
            second
                .luma
                .iter()
                .any(|code| usize::from(*code) < codes / 16)
        );
        assert!(
            second
                .luma
                .iter()
                .any(|code| usize::from(*code) >= codes * 15 / 16)
        );
    }
}
