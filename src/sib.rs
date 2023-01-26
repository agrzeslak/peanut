use bitmaps::Bitmap;

use crate::error::Error;

pub enum Scale {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

///   7                           0
/// +---+---+---+---+---+---+---+---+
/// | scale |   index   |    base   |
/// +---+---+---+---+---+---+---+---+
#[derive(Default)]
pub struct SIB {
    scale: Bitmap<2>,
    index: Bitmap<3>,
    base: Bitmap<3>,
}

impl SIB {
    pub fn get_scale(&self) -> Scale {
        match (self.scale.get(1), self.scale.get(0)) {
            (false, false) => Scale::One,
            (false, true) => Scale::Two,
            (true, false) => Scale::Four,
            (true, true) => Scale::Eight,
        }
    }

    pub fn set_scale(&mut self, scale: &Scale) {
        use Scale::*;
        match scale {
            One => {
                self.scale.set(0, false);
                self.scale.set(1, false);
            }
            Two => {
                self.scale.set(0, true);
                self.scale.set(1, false);
            }
            Four => {
                self.scale.set(0, false);
                self.scale.set(1, true);
            }
            Eight => {
                self.scale.set(0, true);
                self.scale.set(1, true);
            }
        }
    }
}
