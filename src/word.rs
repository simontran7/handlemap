use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

pub trait Word:
    Copy
    + Default
    + Eq
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
    const ONES: Self;

    fn count_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;

    fn checked_shl(self, n: u32) -> Option<Self>;
    fn checked_shr(self, n: u32) -> Option<Self>;
}

macro_rules! impl_word {
    ($($t:ty),* $(,)?) => {
        $(
            impl Word for $t {
                const BITS: usize = <$t>::BITS as usize;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const ONES: Self = !0;

                #[inline] fn count_ones(self) -> u32 { <$t>::count_ones(self) }
                #[inline] fn trailing_zeros(self) -> u32 { <$t>::trailing_zeros(self) }
                #[inline] fn leading_zeros(self) -> u32 { <$t>::leading_zeros(self) }
                #[inline] fn checked_shl(self, n: u32) -> Option<Self> { <$t>::checked_shl(self, n) }
                #[inline] fn checked_shr(self, n: u32) -> Option<Self> { <$t>::checked_shr(self, n) }
            }
        )*
    };
}

impl_word!(u8, u16, u32, u64, u128, usize);
