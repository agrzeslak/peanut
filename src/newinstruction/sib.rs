use bitmaps::Bitmap;

pub enum Scale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

///   7   6   5   4   3   2   1   0
/// +---+---+---+---+---+---+---+---+
/// | scale |   index   |    base   |
/// +---+---+---+---+---+---+---+---+
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
