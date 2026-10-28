use crate::link::{MEMORY_INFO_FIELDS, SOCKET_DROPS_FIELD};

pub(crate) fn socket_drops_in(
    report: &[u32; MEMORY_INFO_FIELDS],
    filled_bytes: usize,
) -> Option<u32> {
    let needed_bytes = (SOCKET_DROPS_FIELD + 1) * size_of::<u32>();

    (filled_bytes >= needed_bytes).then_some(report[SOCKET_DROPS_FIELD])
}
