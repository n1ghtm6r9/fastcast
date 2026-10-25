use crate::gf::build_exp_table;

pub static GF_EXP: [u8; 512] = build_exp_table();
