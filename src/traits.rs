use std::mem;

use num_traits::PrimInt;

pub(crate) trait BitIndex {
    fn bit_at_index(self, index: u32) -> bool;
}

impl<T: PrimInt> BitIndex for T {
    fn bit_at_index(self, index: u32) -> bool {
        self.unsigned_shr(index) & T::one() > T::zero()
    }
}

pub(crate) trait MostSignificantBit {
    fn most_significant_bit(self) -> bool;
}

impl<T: PrimInt> MostSignificantBit for T {
    fn most_significant_bit(self) -> bool {
        let num_bits = mem::size_of::<T>() * 8;
        (self >> num_bits - 1) & T::one() > T::zero()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

pub(crate) trait Signed {
    fn sign(self) -> Sign;
}

impl<T: PrimInt> Signed for T {
    fn sign(self) -> Sign {
        match self.most_significant_bit() {
            false => Sign::Positive,
            true => Sign::Negative,
        }
    }
}
