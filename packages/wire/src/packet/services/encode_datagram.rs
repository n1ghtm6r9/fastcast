use crate::packet::{Datagram, write_datagram};

pub fn encode_datagram(datagram: &Datagram<'_>, out: &mut Vec<u8>) {
    out.clear();
    write_datagram(datagram, out);
}
