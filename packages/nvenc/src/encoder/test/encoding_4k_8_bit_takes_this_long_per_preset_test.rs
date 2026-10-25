use super::benchmark_presets;

#[test]
#[ignore = "benchmark"]
fn encoding_4k_8_bit_takes_this_long_per_preset() {
    benchmark_presets(8);
}
