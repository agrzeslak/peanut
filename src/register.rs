use std::u32;

use bitmaps::Bitmap;
use paste::paste;

use crate::{error::Error, instruction::NasmStr};

trait HighLowBytes32 {
    fn get_high(&self) -> u16;
    fn set_high(&mut self, value: u16);
    fn get_low_high(&self) -> u8;
    fn set_low_high(&mut self, value: u8);
    fn get_low_low(&self) -> u8;
    fn set_low_low(&mut self, value: u8);
}

impl HighLowBytes32 for u32 {
    fn get_high(&self) -> u16 {
        (*self >> 16) as u16
    }

    fn set_high(&mut self, value: u16) {
        *self &= 0x0000ffff;
        *self |= (value as u32) << 16
    }

    fn get_low_high(&self) -> u8 {
        (*self >> 8) as u8
    }

    fn set_low_high(&mut self, value: u8) {
        *self &= 0xffff00ff;
        *self |= (value as u32) << 8
    }

    fn get_low_low(&self) -> u8 {
        *self as u8
    }

    fn set_low_low(&mut self, value: u8) {
        *self &= 0xffffff00;
        *self |= value as u32;
    }
}

pub enum CurrentPrivilegeLevel {
    CPL0,
    CPL1,
    CPL2,
    CPL3,
}

/// Bit #   Mask    Abbreviation    Description     Category    =1              =0
/// 0       0x0001  CF              Carry Flag      Status      CY (Carry)      NC (No Carry)
/// 1       0x0002  Reserved, always 1 in EFLAGS.
/// 2       0x0004  PF              Parity Flag     Status      PE (Parity Even)
#[derive(Default)]
pub struct Eflags(Bitmap<32>);

impl Eflags {
    // FIXME: Make this a macro.
    pub fn get_cf(&self) -> bool {
        self.0.get(0)
    }
    pub fn set_cf(&mut self) {
        self.0.set(0, true);
    }
    pub fn clear_cf(&mut self) {
        self.0.set(0, false);
    }
    pub fn get_pf(&self) -> bool {
        self.0.get(2)
    }
    pub fn set_pf(&mut self) {
        self.0.set(2, true);
    }
    pub fn clear_pf(&mut self) {
        self.0.set(2, false);
    }
    pub fn get_af(&self) -> bool {
        self.0.get(4)
    }
    pub fn set_af(&mut self) {
        self.0.set(4, true);
    }
    pub fn clear_af(&mut self) {
        self.0.set(4, false);
    }
    pub fn get_zf(&self) -> bool {
        self.0.get(6)
    }
    pub fn set_zf(&mut self) {
        self.0.set(6, true);
    }
    pub fn clear_zf(&mut self) {
        self.0.set(6, false);
    }
    pub fn get_sf(&self) -> bool {
        self.0.get(7)
    }
    pub fn set_sf(&mut self) {
        self.0.set(7, true);
    }
    pub fn clear_sf(&mut self) {
        self.0.set(7, false);
    }
    pub fn get_tf(&self) -> bool {
        self.0.get(8)
    }
    pub fn set_tf(&mut self) {
        self.0.set(8, true);
    }
    pub fn clear_tf(&mut self) {
        self.0.set(8, false);
    }
    pub fn get_if(&self) -> bool {
        self.0.get(9)
    }
    pub fn set_if(&mut self) {
        self.0.set(9, true);
    }
    pub fn clear_if(&mut self) {
        self.0.set(9, false);
    }
    pub fn get_df(&self) -> bool {
        self.0.get(10)
    }
    pub fn set_df(&mut self) {
        self.0.set(10, true);
    }
    pub fn clear_df(&mut self) {
        self.0.set(10, false);
    }
    pub fn get_of(&self) -> bool {
        self.0.get(11)
    }
    pub fn set_of(&mut self) {
        self.0.set(11, true);
    }
    pub fn clear_of(&mut self) {
        self.0.set(11, false);
    }
    pub fn get_iopl(&self) -> CurrentPrivilegeLevel {
        let first_bit = self.0.get(12);
        let second_bit = self.0.get(13);
        // FIXME: Verify that these bits correspond to the correct privilege levels.
        match (second_bit, first_bit) {
            (false, false) => CurrentPrivilegeLevel::CPL0,
            (false, true) => CurrentPrivilegeLevel::CPL1,
            (true, false) => CurrentPrivilegeLevel::CPL2,
            (true, true) => CurrentPrivilegeLevel::CPL3,
        }
    }
    pub fn set_iopl(&mut self, cpl: CurrentPrivilegeLevel) {
        // FIXME: Verify that these bits correspond to the correct privilege levels.
        let (second_bit, first_bit) = match cpl {
            CurrentPrivilegeLevel::CPL0 => (false, false),
            CurrentPrivilegeLevel::CPL1 => (false, true),
            CurrentPrivilegeLevel::CPL2 => (true, false),
            CurrentPrivilegeLevel::CPL3 => (true, true),
        };
        self.0.set(12, first_bit);
        self.0.set(13, second_bit);
    }
    pub fn get_nt(&self) -> bool {
        self.0.get(14)
    }
    pub fn set_nt(&mut self) {
        self.0.set(14, true);
    }
    pub fn clear_nt(&mut self) {
        self.0.set(14, false);
    }
    pub fn get_rf(&self) -> bool {
        self.0.get(16)
    }
    pub fn set_rf(&mut self) {
        self.0.set(16, true);
    }
    pub fn clear_rf(&mut self) {
        self.0.set(16, false);
    }
    pub fn get_vm(&self) -> bool {
        self.0.get(17)
    }
    pub fn set_vm(&mut self) {
        self.0.set(17, true);
    }
    pub fn clear_vm(&mut self) {
        self.0.set(17, false);
    }
    pub fn get_ac(&self) -> bool {
        self.0.get(18)
    }
    pub fn set_ac(&mut self) {
        self.0.set(18, true);
    }
    pub fn clear_ac(&mut self) {
        self.0.set(18, false);
    }
    pub fn get_vif(&self) -> bool {
        self.0.get(19)
    }
    pub fn set_vif(&mut self) {
        self.0.set(19, true);
    }
    pub fn clear_vif(&mut self) {
        self.0.set(19, false);
    }
    pub fn get_vip(&self) -> bool {
        self.0.get(20)
    }
    pub fn set_vip(&mut self) {
        self.0.set(20, true);
    }
    pub fn clear_vip(&mut self) {
        self.0.set(20, false);
    }
    pub fn get_id(&self) -> bool {
        self.0.get(21)
    }
    pub fn set_id(&mut self) {
        self.0.set(21, true);
    }
    pub fn clear_id(&mut self) {
        self.0.set(21, false);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Eax,

    Ebx,

    Ecx,

    Edx,

    Edi,
    Esi,

    Ebp,
    Esp,

    Eflags,

    Eip,
    Cs,
    Ds,
    Es,
    Fs,
    Gs,
    Ss,
}

impl TryFrom<&NasmStr<'_>> for Register {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        match value.0.to_lowercase().as_str() {
            "eax" => Ok(Register::Eax),
            "ebx" => Ok(Register::Ebx),
            "ecx" => Ok(Register::Ecx),
            "edx" => Ok(Register::Edx),
            "edi" => Ok(Register::Edi),
            "esi" => Ok(Register::Esi),
            "ebp" => Ok(Register::Ebp),
            "esp" => Ok(Register::Esp),
            "eflags" => Ok(Register::Eflags),
            "eip" => Ok(Register::Eip),
            "cs" => Ok(Register::Cs),
            "ds" => Ok(Register::Ds),
            "es" => Ok(Register::Es),
            "fs" => Ok(Register::Fs),
            "gs" => Ok(Register::Gs),
            "ss" => Ok(Register::Ss),
            _ => Err(Error::CannotCovertType(format!("{} is not a valid register", value.0)))
        }
    }
}

#[derive(Default)]
pub struct Registers {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    eflags: Eflags,
    eip: u32,
    cs: u16,
    ds: u16,
    es: u16,
    fs: u16,
    gs: u16,
    ss: u16,
}

macro_rules! create_general_register_accessors {
    ($register_letter:ident) => {
        paste! {
            pub fn [<get_e $register_letter x>](&self) -> u32 {
                self.[<e $register_letter x>]
            }

            pub fn [<set_e $register_letter x>](&mut self, value: u32) {
                self.[<e $register_letter x>] = value;
            }

            pub fn [<get_ $register_letter x>](&self) -> u16 {
                self.[<e $register_letter x>].get_high()
            }

            pub fn [<set_ $register_letter x>](&mut self, value: u16) {
                self.[<e $register_letter x>].set_high(value)
            }

            pub fn [<get_ $register_letter h>](&self) -> u8 {
                self.[<e $register_letter x>].get_low_high()
            }

            pub fn [<set_ $register_letter h>](&mut self, value: u8) {
                self.[<e $register_letter x>].set_low_high(value);
            }

            pub fn [<get_ $register_letter l>](&self) -> u8 {
                self.[<e $register_letter x>].get_low_low()
            }

            pub fn [<set_ $register_letter l>](&mut self, value: u8) {
                self.[<e $register_letter x>].set_low_low(value);
            }
        }
    };
}
impl Registers {
    create_general_register_accessors!(a);
    create_general_register_accessors!(b);
    create_general_register_accessors!(c);
    create_general_register_accessors!(d);

    pub fn get_edi(&self) -> u32 {
        self.edi
    }

    pub fn set_edi(&mut self, value: u32) {
        self.edi = value
    }

    pub fn get_esi(&self) -> u32 {
        self.esi
    }

    pub fn set_esi(&mut self, value: u32) {
        self.esi = value
    }

    pub fn get_ebp(&self) -> u32 {
        self.ebp
    }
    pub fn set_ebp(&mut self, value: u32) {
        self.ebp = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the getters and setters of a general register. `$register_letter` is expanded to form
    /// `e $register_letter x`, e.g. eax.
    macro_rules! test_general_register_accessors {
        ($register_letter:ident) => {
            paste! {
                let mut registers = Registers::default();
                registers.[<set_e $register_letter x>](0xdeadc0de);
                assert_eq!(registers.[<get_e $register_letter x>](), 0xdeadc0de);
                assert_eq!(registers.[<get_ $register_letter x>](), 0xdead);
                assert_eq!(registers.[<get_ $register_letter h>](), 0xc0);
                assert_eq!(registers.[<get_ $register_letter l>](), 0xde);

                registers.[<set_ $register_letter x>](0xc0de);
                registers.[<set_ $register_letter h>](0xb3);
                registers.[<set_ $register_letter l>](0x3f);
                assert_eq!(registers.[<get_e $register_letter x>](), 0xc0deb33f as u32);
                assert_eq!(registers.[<get_ $register_letter x>](), 0xc0de);
                assert_eq!(registers.[<get_ $register_letter h>](), 0xb3);
                assert_eq!(registers.[<get_ $register_letter l>](), 0x3f);
            }
        };
    }

    #[test]
    fn eax_get_and_set() {
        test_general_register_accessors!(a);
    }

    #[test]
    fn ebx_get_and_set() {
        test_general_register_accessors!(b);
    }

    #[test]
    fn ecx_get_and_set() {
        test_general_register_accessors!(c);
    }

    #[test]
    fn edx_get_and_set() {
        test_general_register_accessors!(d);
    }
}
