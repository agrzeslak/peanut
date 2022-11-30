pub(crate) trait LeastSignificantByte {
    fn least_significant_byte(&self) -> u8;
}

impl LeastSignificantByte for u8 {
    fn least_significant_byte(&self) -> u8 {
        *self
    }
}

impl LeastSignificantByte for u16 {
    fn least_significant_byte(&self) -> u8 {
        self.to_le_bytes()[0]
    }
}

impl LeastSignificantByte for u32 {
    fn least_significant_byte(&self) -> u8 {
        self.to_le_bytes()[0]
    }
}

impl LeastSignificantByte for u64 {
    fn least_significant_byte(&self) -> u8 {
        self.to_le_bytes()[0]
    }
}

impl LeastSignificantByte for u128 {
    fn least_significant_byte(&self) -> u8 {
        self.to_le_bytes()[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn least_significant_byte() {
        assert_eq!(u8::MAX.least_significant_byte(), u8::MAX);
        assert_eq!(u8::MIN.least_significant_byte(), u8::MIN);

        assert_eq!(u16::MAX.least_significant_byte(), u8::MAX);
        assert_eq!(u16::MIN.least_significant_byte(), u8::MIN);

        assert_eq!(u32::MAX.least_significant_byte(), u8::MAX);
        assert_eq!(u32::MIN.least_significant_byte(), u8::MIN);
        
        assert_eq!(u64::MAX.least_significant_byte(), u8::MAX);
        assert_eq!(u64::MIN.least_significant_byte(), u8::MIN);

        assert_eq!(u128::MAX.least_significant_byte(), u8::MAX);
        assert_eq!(u128::MIN.least_significant_byte(), u8::MIN);
    }
}
