use super::BitWriter;

impl BitWriter {
    pub fn write_se(&mut self, value: i64) {
        let code = if value > 0 { 2 * value - 1 } else { -2 * value };

        self.write_ue(u32::try_from(code).expect("the signed value fits an exp-Golomb code"));
    }
}
