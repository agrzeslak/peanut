use bitmaps::Bitmap;

pub enum Scale {
    One = 0b00,
    Two = 0b01,
    Four = 0b10,
    Eight = 0b11,
}

pub enum Index {
    Eax = 0b000,
    Ecx = 0b001,
    Edx = 0b010,
    Ebx = 0b011,
    Ebp = 0b101,
    Esi = 0b110,
    Edi = 0b111,
}

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
pub struct SIB(Bitmap<8>);

impl SIB {
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
}
