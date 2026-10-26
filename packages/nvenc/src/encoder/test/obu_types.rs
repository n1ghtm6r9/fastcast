use super::obus;

pub fn obu_types(temporal_unit: &[u8]) -> Vec<u8> {
    obus(temporal_unit)
        .into_iter()
        .map(|(kind, _)| kind)
        .collect()
}
