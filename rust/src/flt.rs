use std::{fmt, ops::{AddAssign, Range}};

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct HashFlt(pub u32);

impl HashFlt {
    #[inline(always)]
    pub const fn new(x: f32) -> Self {
        Self(f32::to_bits(x))
    }

    #[inline(always)]
    pub fn rand(r: Range<f32>) -> Self {
        let u = rand::random_range(r);
        Self::new(u)
    }

    #[inline(always)]
    pub fn f32(&self) -> f32 {
        Into::<f32>::into(*self)
    }
}

impl fmt::Debug for HashFlt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0f{}", self.f32())
    }
}

impl Into<f32> for HashFlt {
    #[inline(always)]
    fn into(self) -> f32 {
        f32::from_bits(self.0)
    }
}

impl AddAssign<f32> for HashFlt {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        *self = HashFlt::new(self.f32() + rhs);
    }
}
