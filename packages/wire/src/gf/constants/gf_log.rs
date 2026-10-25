use crate::gf::build_log_table;

pub static GF_LOG: [u8; 256] = build_log_table();
