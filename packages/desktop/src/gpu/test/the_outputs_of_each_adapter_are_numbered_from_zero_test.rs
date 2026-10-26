use crate::gpu::list_outputs;
use std::collections::BTreeSet;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_outputs_of_each_adapter_are_numbered_from_zero() {
    let outputs = list_outputs().unwrap();
    let adapters: BTreeSet<u32> = outputs.iter().map(|output| output.adapter_index).collect();

    for adapter in adapters {
        let numbers: Vec<u32> = outputs
            .iter()
            .filter(|output| output.adapter_index == adapter)
            .map(|output| output.output_index)
            .collect();
        let expected: Vec<u32> = (0..).take(numbers.len()).collect();

        assert_eq!(numbers, expected);
    }
}
