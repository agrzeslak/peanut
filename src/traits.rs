use std::mem;

use num_traits::{PrimInt, Unsigned};

pub(crate) trait BitIndex {
    fn bit_at_index(self, index: u32) -> bool;
}

impl<T: PrimInt> BitIndex for T {
    fn bit_at_index(self, index: u32) -> bool {
        self.unsigned_shr(index) & T::one() > T::zero()
    }
}

pub(crate) trait HighLowBytes32 {
    fn get_low_16(&self) -> u16;
    fn set_low_16(&mut self, value: u16);
    fn get_high_8(&self) -> u8;
    fn set_high_8(&mut self, value: u8);
    fn get_low_8(&self) -> u8;
    fn set_low_8(&mut self, value: u8);
}

impl HighLowBytes32 for u32 {
    fn get_low_16(&self) -> u16 {
        *self as u16
    }

    fn set_low_16(&mut self, value: u16) {
        *self &= 0xffff0000;
        *self |= value as u32
    }

    fn get_high_8(&self) -> u8 {
        (*self >> 8) as u8
    }

    fn set_high_8(&mut self, value: u8) {
        *self &= 0xffff00ff;
        *self |= (value as u32) << 8
    }

    fn get_low_8(&self) -> u8 {
        *self as u8
    }

    fn set_low_8(&mut self, value: u8) {
        *self &= 0xffffff00;
        *self |= value as u32;
    }
}

pub(crate) trait MostSignificantBit {
    fn most_significant_bit(&self) -> bool;
}

impl<T: PrimInt> MostSignificantBit for T {
    fn most_significant_bit(&self) -> bool {
        let num_bits = mem::size_of::<T>() * 8;
        (*self >> num_bits - 1) & T::one() > T::zero()
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

pub(crate) trait AsUnsigned {
    type Unsigned: PrimInt + Unsigned;

    fn as_unsigned(self) -> Self::Unsigned;
}

macro_rules! impl_as_unsigned {
    ($signed:ty, $unsigned:ty) => {
        impl AsUnsigned for $signed {
            type Unsigned = $unsigned;

            fn as_unsigned(self) -> Self::Unsigned {
                self as $unsigned
            }
        }
    };
    ($unsigned:ty) => {
        impl AsUnsigned for $unsigned {
            type Unsigned = Self;

            fn as_unsigned(self) -> Self::Unsigned {
                self
            }
        }
    }
}

impl_as_unsigned!(u8);
impl_as_unsigned!(u16);
impl_as_unsigned!(u32);
impl_as_unsigned!(u64);
impl_as_unsigned!(u128);
impl_as_unsigned!(usize);

impl_as_unsigned!(i8, u8);
impl_as_unsigned!(i16, u16);
impl_as_unsigned!(i32, u32);
impl_as_unsigned!(i64, u64);
impl_as_unsigned!(i128, u128);
impl_as_unsigned!(isize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_as_unsigned {
        ($source_type:ty, $target_type:ty) => {
            let value: $source_type = 0;
            assert_eq!(value as $target_type, value.as_unsigned());
            let value = <$source_type>::MIN;
            assert_eq!(value as $target_type, value.as_unsigned());
            let value = <$source_type>::MAX;
            assert_eq!(value as $target_type, value.as_unsigned());
        };
    }

    #[test]
    fn as_unsigned() {
        test_as_unsigned!(u8, u8);
        test_as_unsigned!(i8, u8);
        test_as_unsigned!(u16, u16);
        test_as_unsigned!(i16, u16);
        test_as_unsigned!(u32, u32);
        test_as_unsigned!(i32, u32);
        test_as_unsigned!(u64, u64);
        test_as_unsigned!(i64, u64);
        test_as_unsigned!(u128, u128);
        test_as_unsigned!(i128, u128);
    }
}
