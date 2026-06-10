use std::{ops::Range, fmt};

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct HashFlt(pub u32);

impl HashFlt {
    #[inline(always)]
    pub fn new(x: f32) -> Self {
        Self(f32::to_bits(x))
    }

    #[inline(always)]
    pub fn rand(r: Range<f32>) -> Self {
        let u = rand::random_range(r);
        Self::new(u)
    }
}

impl fmt::Debug for HashFlt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0f{}", f32::from_bits(self.0))
    }
}
