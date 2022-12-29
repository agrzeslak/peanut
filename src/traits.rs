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
