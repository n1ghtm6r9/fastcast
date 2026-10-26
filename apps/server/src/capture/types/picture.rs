#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Picture {
    #[default]
    Missing,
    Converted,
    Waiting,
}
