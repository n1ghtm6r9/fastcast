pub fn white_over_black() -> Vec<u8> {
    let white = [255, 255, 255, 255];
    let black = [0, 0, 0, 255];

    [white, white, black, black].concat()
}
