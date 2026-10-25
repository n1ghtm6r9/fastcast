pub fn monochrome_bit(row: &[u8], column: usize) -> bool {
    (row[column / 8] >> (7 - column % 8)) & 1 == 1
}
