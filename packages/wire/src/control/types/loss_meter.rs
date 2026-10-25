use crate::control::ReceiveCounters;

#[derive(Debug, Clone, Copy, Default)]
pub struct LossMeter {
    pub(crate) previous: Option<ReceiveCounters>,
}
