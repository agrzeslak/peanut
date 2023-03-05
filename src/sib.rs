use bitmaps::Bitmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    One = 0b00,
    Two = 0b01,
    Four = 0b10,
    Eight = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Index {
    Eax = 0b000,
    Ecx = 0b001,
    Edx = 0b010,
    Ebx = 0b011,
    Ebp = 0b101,
    Esi = 0b110,
    Edi = 0b111,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Eax = 0b000,
    Ecx = 0b001,
    Edx = 0b010,
    Ebx = 0b011,
    Esp = 0b100,
    DisplacementOnlyOrEbp = 0b101,
    Esi = 0b110,
    Edi = 0b111,
}

///   7   6   5   4   3   2   1   0
/// +---+---+---+---+---+---+---+---+
/// | scale |   index   |    base   |
/// +---+---+---+---+---+---+---+---+
/// http://www.c-jump.com/CIS77/CPU/x86/X77_0100_sib_byte_layout.htm
// TODO: Are the values encoded left-to-right or right-to-left?
#[derive(Default)]
pub struct SIB(Bitmap<8>);

// TODO: Tests
impl SIB {
    pub fn new(scale: &Scale, index: &Index, base: &Base) -> Self {
        let mut sib = SIB::default();
        sib.set_scale(scale);
        sib.set_index(index);
        sib.set_base(base);
        sib
    }

    pub fn get_scale(&self) -> Scale {
        match (self.0.get(7), self.0.get(6)) {
            (false, false) => Scale::One,
            (false, true) => Scale::Two,
            (true, false) => Scale::Four,
            (true, true) => Scale::Eight,
        }
    }

    pub fn set_scale(&mut self, scale: &Scale) {
        match scale {
            Scale::One => {
                self.0.set(6, false);
                self.0.set(7, false);
            }
            Scale::Two => {
                self.0.set(6, true);
                self.0.set(7, false);
            }
            Scale::Four => {
                self.0.set(6, false);
                self.0.set(7, true);
            }
            Scale::Eight => {
                self.0.set(6, true);
                self.0.set(7, true);
            }
        }
    }

    pub fn get_index(&self) -> Index {
        match (self.0.get(5), self.0.get(4), self.0.get(3)) {
            (false, false, false) => Index::Eax,
            (false, false, true) => Index::Ecx,
            (false, true, false) => Index::Edx,
            (false, true, true) => Index::Ebx,
            (true, false, false) => unreachable!(),
            (true, false, true) => Index::Ebp,
            (true, true, false) => Index::Esi,
            (true, true, true) => Index::Edi,
        }
    }

    pub fn set_index(&mut self, index: &Index) {
        let bits = match index {
            Index::Eax => (false, false, false),
            Index::Ecx => (false, false, true),
            Index::Edx => (false, true, false),
            Index::Ebx => (false, true, true),
            Index::Ebp => (true, false, true),
            Index::Esi => (true, true, false),
            Index::Edi => (true, true, true),
        };
        self.0.set(5, bits.0);
        self.0.set(4, bits.1);
        self.0.set(3, bits.2);
    }

    pub fn get_base(&self) -> Base {
        match (self.0.get(2), self.0.get(1), self.0.get(0)) {
            (false, false, false) => Base::Eax,
            (false, false, true) => Base::Ecx,
            (false, true, false) => Base::Edx,
            (false, true, true) => Base::Ebx,
            (true, false, false) => Base::Esp,
            (true, false, true) => Base::DisplacementOnlyOrEbp,
            (true, true, false) => Base::Esi,
            (true, true, true) => Base::Edi,
        }
    }

    pub fn set_base(&mut self, base: &Base) {
        let bits = match base {
            Base::Eax => (false, false, false),
            Base::Ecx => (false, false, true),
            Base::Edx => (false, true, false),
            Base::Ebx => (false, true, true),
            Base::Esp => (true, false, false),
            Base::DisplacementOnlyOrEbp => (true, false, true),
            Base::Esi => (true, true, false),
            Base::Edi => (true, true, true),
        };
        self.0.set(2, bits.0);
        self.0.set(1, bits.1);
        self.0.set(0, bits.2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sib = SIB::new(&Scale::Two, &Index::Ecx, &Base::Edx);
        assert_eq!(sib.get_scale(), Scale::Two);
        assert_eq!(sib.get_index(), Index::Ecx);
        assert_eq!(sib.get_base(), Base::Edx);
    }

    #[test]
    fn scale() {
        let mut sib = SIB::default();
        sib.set_scale(&Scale::Eight);
        assert_eq!(sib.get_scale(), Scale::Eight);
        sib.set_scale(&Scale::Four);
        assert_eq!(sib.get_scale(), Scale::Four);
        sib.set_scale(&Scale::Two);
        assert_eq!(sib.get_scale(), Scale::Two);
        sib.set_scale(&Scale::One);
        assert_eq!(sib.get_scale(), Scale::One);
    }

    fn index() {
        let mut sib = SIB::default();
        sib.set_index(&Index::Edi);
        assert_eq!(sib.get_index(), Index::Edi);
        sib.set_index(&Index::Esi);
        assert_eq!(sib.get_index(), Index::Esi);
        sib.set_index(&Index::Ebp);
        assert_eq!(sib.get_index(), Index::Ebp);
        sib.set_index(&Index::Ebx);
        assert_eq!(sib.get_index(), Index::Ebx);
        sib.set_index(&Index::Edx);
        assert_eq!(sib.get_index(), Index::Edx);
        sib.set_index(&Index::Ecx);
        assert_eq!(sib.get_index(), Index::Ecx);
        sib.set_index(&Index::Eax);
        assert_eq!(sib.get_index(), Index::Eax);
    }

    fn base() {
        let mut sib = SIB::default();
        sib.set_base(&Base::Edi);
        assert_eq!(sib.get_base(), Base::Edi);
        sib.set_base(&Base::Esi);
        assert_eq!(sib.get_base(), Base::Esi);
        sib.set_base(&Base::DisplacementOnlyOrEbp);
        assert_eq!(sib.get_base(), Base::DisplacementOnlyOrEbp);
        sib.set_base(&Base::Esp);
        assert_eq!(sib.get_base(), Base::Esp);
        sib.set_base(&Base::Ebx);
        assert_eq!(sib.get_base(), Base::Ebx);
        sib.set_base(&Base::Edx);
        assert_eq!(sib.get_base(), Base::Edx);
        sib.set_base(&Base::Ecx);
        assert_eq!(sib.get_base(), Base::Ecx);
        sib.set_base(&Base::Eax);
        assert_eq!(sib.get_base(), Base::Eax);
    }
}
