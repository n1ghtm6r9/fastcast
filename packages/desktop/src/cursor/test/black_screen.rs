pub fn black_screen(width: usize, height: usize) -> Vec<u8> {
    [0, 0, 0, 255].repeat(width * height)
}
