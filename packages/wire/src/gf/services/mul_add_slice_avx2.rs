use crate::gf::{
    HIGH_NIBBLE_PRODUCTS, LOW_NIBBLE_PRODUCTS, NIBBLE_BITS, NIBBLE_MASK, mul_add_slice_ssse3,
};
use std::arch::x86_64::{
    _mm_loadu_si128, _mm256_and_si256, _mm256_broadcastsi128_si256, _mm256_loadu_si256,
    _mm256_set1_epi8, _mm256_shuffle_epi8, _mm256_srli_epi16, _mm256_storeu_si256,
    _mm256_xor_si256,
};

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn mul_add_slice_avx2(coefficient: u8, source: &[u8], target: &mut [u8]) {
    if coefficient == 0 {
        return;
    }

    let length = source.len().min(target.len());
    let low_table = _mm256_broadcastsi128_si256(unsafe {
        _mm_loadu_si128(LOW_NIBBLE_PRODUCTS[coefficient as usize].as_ptr().cast())
    });
    let high_table = _mm256_broadcastsi128_si256(unsafe {
        _mm_loadu_si128(HIGH_NIBBLE_PRODUCTS[coefficient as usize].as_ptr().cast())
    });
    let mask = _mm256_set1_epi8(NIBBLE_MASK as i8);
    let (source_chunks, source_tail) = source[..length].as_chunks::<32>();
    let (target_chunks, target_tail) = target[..length].as_chunks_mut::<32>();

    for (source_chunk, target_chunk) in source_chunks.iter().zip(target_chunks) {
        let bytes = unsafe { _mm256_loadu_si256(source_chunk.as_ptr().cast()) };
        let low_nibbles = _mm256_and_si256(bytes, mask);
        let high_nibbles = _mm256_and_si256(_mm256_srli_epi16::<NIBBLE_BITS>(bytes), mask);
        let products = _mm256_xor_si256(
            _mm256_shuffle_epi8(low_table, low_nibbles),
            _mm256_shuffle_epi8(high_table, high_nibbles),
        );
        let accumulated = unsafe { _mm256_loadu_si256(target_chunk.as_ptr().cast()) };

        unsafe {
            _mm256_storeu_si256(
                target_chunk.as_mut_ptr().cast(),
                _mm256_xor_si256(accumulated, products),
            )
        };
    }

    unsafe { mul_add_slice_ssse3(coefficient, source_tail, target_tail) };
}
