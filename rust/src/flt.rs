use rand::Rng;

#[inline(always)]
fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = f64::to_bits(val);
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct HashFlt(pub u64, pub i16, pub i8);

impl HashFlt {
    #[inline(always)]
    pub fn new(x: f32) -> Self {
        let (m, e, s) = integer_decode(x as f64);
        Self(m, e, s)
    }

    #[inline(always)]
    pub fn rand(mut rng: impl Rng) -> Self {
        let u = rng.next_u32();
        let f = f32::from_bits(u);
        Self::new(f)
    }
}
