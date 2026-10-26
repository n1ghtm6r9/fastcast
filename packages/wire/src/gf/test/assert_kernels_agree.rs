use crate::gf::mul_add_slice_scalar;
use crate::gf::test::available_kernels;

pub fn assert_kernels_agree(
    coefficient: u8,
    source: &[u8],
    initial: &[u8],
    target_offset: usize,
) -> usize {
    let mut expected = initial.to_vec();

    mul_add_slice_scalar(coefficient, source, &mut expected);

    let kernels = available_kernels();

    for (name, kernel) in &kernels {
        let mut buffer = vec![0u8; target_offset];

        buffer.extend_from_slice(initial);
        unsafe { kernel(coefficient, source, &mut buffer[target_offset..]) };

        assert_eq!(
            buffer[target_offset..],
            expected[..],
            "{name} kernel, coefficient {coefficient}, source length {}, target length {}, target offset {target_offset}",
            source.len(),
            initial.len(),
        );
    }

    kernels.len()
}
