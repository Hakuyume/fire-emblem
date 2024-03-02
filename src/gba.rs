#[derive(Clone, Copy)]
pub struct Rng(u16, u16, u16);

impl Rng {
    pub fn gen(&mut self) -> u8 {
        *self = Self(
            (self.0 >> 5).wrapping_add(self.1 << 11) ^ (self.2 << 1).saturating_add(self.1 >> 15),
            self.0,
            self.1,
        );
        ((self.0 as u32 * 100) >> 16) as _
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self(0x3671, 0x90ea, 0x1496)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rng() {
        let mut rng = super::Rng::default();
        assert_eq!(rng.gen(), 47);
        assert_eq!(rng.gen(), 66);
        assert_eq!(rng.gen(), 60);
        assert_eq!(rng.gen(), 46);
        assert_eq!(rng.gen(), 77);
        assert_eq!(rng.gen(), 70);
        assert_eq!(rng.gen(), 61);
        assert_eq!(rng.gen(), 29);
    }
}
