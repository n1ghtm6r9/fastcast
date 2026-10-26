#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NalUnitKind {
    Vps,
    Sps,
    Pps,
    AccessUnitDelimiter,
    Other,
}
