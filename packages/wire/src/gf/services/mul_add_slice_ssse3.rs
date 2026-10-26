use crate::gf::{
    HIGH_NIBBLE_PRODUCTS, LOW_NIBBLE_PRODUCTS, NIBBLE_BITS, NIBBLE_MASK, mul_add_tail,
};
use std::arch::x86_64::{
    _mm_and_si128, _mm_loadu_si128, _mm_set1_epi8, _mm_shuffle_epi8, _mm_srli_epi16,
    _mm_storeu_si128, _mm_xor_si128,
};

#[target_feature(enable = "ssse3")]
pub(crate) unsafe fn mul_add_slice_ssse3(coefficient: u8, source: &[u8], target: &mut [u8]) {
    if coefficient == 0 {
        return;
    }

    let length = source.len().min(target.len());
    let low_products = &LOW_NIBBLE_PRODUCTS[coefficient as usize];
    let high_products = &HIGH_NIBBLE_PRODUCTS[coefficient as usize];
    let low_table = unsafe { _mm_loadu_si128(low_products.as_ptr().cast()) };
    let high_table = unsafe { _mm_loadu_si128(high_products.as_ptr().cast()) };
    let mask = _mm_set1_epi8(NIBBLE_MASK as i8);
    let (source_chunks, source_tail) = source[..length].as_chunks::<16>();
    let (target_chunks, target_tail) = target[..length].as_chunks_mut::<16>();

    for (source_chunk, target_chunk) in source_chunks.iter().zip(target_chunks) {
        let bytes = unsafe { _mm_loadu_si128(source_chunk.as_ptr().cast()) };
        let low_nibbles = _mm_and_si128(bytes, mask);
        let high_nibbles = _mm_and_si128(_mm_srli_epi16::<NIBBLE_BITS>(bytes), mask);
        let products = _mm_xor_si128(
            _mm_shuffle_epi8(low_table, low_nibbles),
            _mm_shuffle_epi8(high_table, high_nibbles),
        );
        let accumulated = unsafe { _mm_loadu_si128(target_chunk.as_ptr().cast()) };

        unsafe {
            _mm_storeu_si128(
                target_chunk.as_mut_ptr().cast(),
                _mm_xor_si128(accumulated, products),
            )
        };
    }

    mul_add_tail(low_products, high_products, source_tail, target_tail);
}
