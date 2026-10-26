use crate::gf::MulAddKernel;
use std::sync::OnceLock;

pub static MUL_ADD_KERNEL: OnceLock<MulAddKernel> = OnceLock::new();
