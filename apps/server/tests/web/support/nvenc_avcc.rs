use crate::support::hex_bytes;

const AVCC: &str = "01640033ffe1002767640033ac2b20050016b6022000007d00003a981c000003009896800000989683779447c70a9201000468eb8f2cfdf8f800";

pub fn nvenc_avcc() -> Vec<u8> {
    hex_bytes(AVCC)
}
