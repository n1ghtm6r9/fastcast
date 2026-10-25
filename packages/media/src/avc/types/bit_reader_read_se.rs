use crate::avc::BitReader;

impl BitReader<'_> {
    pub fn read_se(&mut self) -> Option<i64> {
        let code = i64::from(self.read_ue()?);

        if code % 2 == 1 {
            Some((code + 1) / 2)
        } else {
            Some(-(code / 2))
        }
    }
}
