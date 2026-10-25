use crate::avc::test::BitWriter;

pub fn write_profile_bits(writer: &mut BitWriter) {
    writer.write_bits(0, 2);
    writer.write_bit(false);
    writer.write_bits(1, 5);
    writer.write_bits(0x6000_0000, 32);
    writer.write_bits(0b1001, 4);
    writer.write_bits(0, 32);
    writer.write_bits(0, 11);
    writer.write_bit(false);
}
