use std::{fmt::Display, u32};

use bitmaps::Bitmap;
use paste::paste;

use crate::{
    error::Error,
    instruction::{NasmStr, Size},
};

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
pub enum Register32 {
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
}

impl Display for Register32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register32::*;

        let register = match self {
            Eax => "EAX",
            Ebx => "EBX",
            Ecx => "ECX",
            Edx => "EDX",
            Edi => "EDI",
            Esi => "ESI",
            Ebp => "EBP",
            Esp => "ESP",
            Eflags => "EFLAGS",
            Eip => "EIP",
        };

        write!(f, "{register}")
    }
}

impl TryFrom<&NasmStr<'_>> for Register32 {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register32::*;

        match value.0.to_lowercase().as_str() {
            "eax" => Ok(Eax),
            "ebx" => Ok(Ebx),
            "ecx" => Ok(Ecx),
            "edx" => Ok(Edx),
            "edi" => Ok(Edi),
            "esi" => Ok(Esi),
            "ebp" => Ok(Ebp),
            "esp" => Ok(Esp),
            "eflags" => Ok(Eflags),
            "eip" => Ok(Eip),
            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid 32-bit register",
                value.0
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register16 {
    Ax,
    Bx,
    Cx,
    Dx,
    Cs,
    Ds,
    Es,
    Fs,
    Gs,
    Ss,
}

impl Display for Register16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register16::*;

        let register = match self {
            Ax => "AX",
            Bx => "BX",
            Cx => "CX",
            Dx => "DX",
            Cs => "CS",
            Ds => "DS",
            Es => "ES",
            Fs => "FS",
            Gs => "GS",
            Ss => "SS",
        };

        write!(f, "{register}")
    }
}

impl TryFrom<&NasmStr<'_>> for Register16 {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register16::*;

        match value.0.to_lowercase().as_str() {
            "ax" => Ok(Ax),
            "bx" => Ok(Bx),
            "cx" => Ok(Cx),
            "dx" => Ok(Dx),
            "cs" => Ok(Cs),
            "ds" => Ok(Ds),
            "es" => Ok(Es),
            "fs" => Ok(Fs),
            "gs" => Ok(Gs),
            "ss" => Ok(Ss),
            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid 16-bit register",
                value.0
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register8 {
    Ah,
    Al,
    Bh,
    Bl,
    Ch,
    Cl,
    Dh,
    Dl,
}

impl Display for Register8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register8::*;

        let register = match self {
            Ah => "AH",
            Al => "AL",
            Bh => "BH",
            Bl => "BL",
            Ch => "CH",
            Cl => "CL",
            Dh => "DH",
            Dl => "DL",
        };

        write!(f, "{register}")
    }
}

impl TryFrom<&NasmStr<'_>> for Register8 {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register8::*;

        match value.0.to_lowercase().as_str() {
            "ah" => Ok(Ah),
            "al" => Ok(Al),
            "bh" => Ok(Bh),
            "bl" => Ok(Bl),
            "ch" => Ok(Ch),
            "cl" => Ok(Cl),
            "dh" => Ok(Dh),
            "dl" => Ok(Dl),
            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid 8-bit register",
                value.0
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Eax,
    Ax,
    Ah,
    Al,

    Ebx,
    Bx,
    Bh,
    Bl,

    Ecx,
    Cx,
    Ch,
    Cl,

    Edx,
    Dx,
    Dh,
    Dl,

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

impl Register {
    pub fn size(&self) -> Size {
        use Register::*;
        use Size::*;

        match self {
            Eax | Ebx | Ecx | Edx | Edi | Esi | Ebp | Esp | Eflags | Eip => Dword,
            Ax | Bx | Cx | Dx | Cs | Ds | Es | Fs | Gs | Ss => Word,
            Ah | Al | Bh | Bl | Ch | Cl | Dh | Dl => Byte,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;

        let register = match self {
            Eax => "EAX",
            Ax => "AX",
            Ah => "AH",
            Al => "AL",

            Ebx => "EBX",
            Bx => "BX",
            Bh => "BH",
            Bl => "BL",

            Ecx => "ECX",
            Cx => "CX",
            Ch => "CH",
            Cl => "CL",

            Edx => "EDX",
            Dx => "DX",
            Dh => "DH",
            Dl => "DL",

            Edi => "EDI",
            Esi => "ESI",
            Ebp => "EBP",
            Esp => "ESP",

            Eflags => "EFLAGS",
            Eip => "EIP",

            Cs => "CS",
            Ds => "DS",
            Es => "ES",
            Fs => "FS",
            Gs => "GS",
            Ss => "SS",
        };

        write!(f, "{register}")
    }
}

impl From<&Register32> for Register {
    fn from(register: &Register32) -> Self {
        match register {
            Register32::Eax => Register::Eax,
            Register32::Ebx => Register::Ebx,
            Register32::Ecx => Register::Ecx,
            Register32::Edx => Register::Edx,
            Register32::Edi => Register::Edi,
            Register32::Esi => Register::Esi,
            Register32::Ebp => Register::Ebp,
            Register32::Esp => Register::Esp,
            Register32::Eflags => Register::Eflags,
            Register32::Eip => Register::Eip,
        }
    }
}

impl From<&Register16> for Register {
    fn from(register: &Register16) -> Self {
        match register {
            Register16::Ax => Register::Ax,
            Register16::Bx => Register::Bx,
            Register16::Cx => Register::Cx,
            Register16::Dx => Register::Dx,
            Register16::Cs => Register::Cs,
            Register16::Ds => Register::Ds,
            Register16::Es => Register::Es,
            Register16::Fs => Register::Fs,
            Register16::Gs => Register::Gs,
            Register16::Ss => Register::Ss,
        }
    }
}

impl From<&Register8> for Register {
    fn from(register: &Register8) -> Self {
        match register {
            Register8::Ah => Register::Ah,
            Register8::Al => Register::Al,
            Register8::Bh => Register::Bh,
            Register8::Bl => Register::Bl,
            Register8::Ch => Register::Ch,
            Register8::Cl => Register::Cl,
            Register8::Dh => Register::Dh,
            Register8::Dl => Register::Dl,
        }
    }
}

impl TryFrom<&NasmStr<'_>> for Register {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register::*;

        match value.0.to_lowercase().as_str() {
            "eax" => Ok(Eax),
            "ax" => Ok(Ax),
            "ah" => Ok(Ah),
            "al" => Ok(Al),

            "ebx" => Ok(Ebx),
            "bx" => Ok(Bx),
            "bh" => Ok(Bh),
            "bl" => Ok(Bl),

            "ecx" => Ok(Ecx),
            "cx" => Ok(Cx),
            "ch" => Ok(Ch),
            "cl" => Ok(Cl),

            "edx" => Ok(Edx),
            "dx" => Ok(Dx),
            "dh" => Ok(Dh),
            "dl" => Ok(Dl),

            "edi" => Ok(Edi),
            "esi" => Ok(Esi),
            "ebp" => Ok(Ebp),
            "esp" => Ok(Esp),

            "eflags" => Ok(Eflags),
            "eip" => Ok(Eip),

            "cs" => Ok(Cs),
            "ds" => Ok(Ds),
            "es" => Ok(Es),
            "fs" => Ok(Fs),
            "gs" => Ok(Gs),
            "ss" => Ok(Ss),
            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid register",
                value.0
            ))),
        }
    }
}

impl TryFrom<&Register> for Register32 {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        match register {
            Register::Eax => Ok(Register32::Eax),
            Register::Ebx => Ok(Register32::Ebx),
            Register::Ecx => Ok(Register32::Ecx),
            Register::Edx => Ok(Register32::Edx),
            Register::Edi => Ok(Register32::Edi),
            Register::Esi => Ok(Register32::Esi),
            Register::Ebp => Ok(Register32::Ebp),
            Register::Esp => Ok(Register32::Esp),
            Register::Eflags => Ok(Register32::Eflags),
            Register::Eip => Ok(Register32::Eip),
            _ => Err(Error::CannotCovertType(format!("{} is not a 32-bit register", register)))
        } 
    }
}

impl TryFrom<&Register> for Register16 {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        match register {
            Register::Ax => Ok(Register16::Ax),
            Register::Bx => Ok(Register16::Bx),
            Register::Cx => Ok(Register16::Cx),
            Register::Dx => Ok(Register16::Dx),
            Register::Cs => Ok(Register16::Cs),
            Register::Ds => Ok(Register16::Ds),
            Register::Es => Ok(Register16::Es),
            Register::Fs => Ok(Register16::Fs),
            Register::Gs => Ok(Register16::Gs),
            Register::Ss => Ok(Register16::Ss),
            _ => Err(Error::CannotCovertType(format!("{} is not a 16-bit register", register)))
        }
    }
}

impl TryFrom<&Register> for Register8 {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        match register {
            Register::Ah => Ok(Register8::Ah),
            Register::Al => Ok(Register8::Al),
            Register::Bh => Ok(Register8::Bh),
            Register::Bl => Ok(Register8::Bl),
            Register::Ch => Ok(Register8::Ch),
            Register::Cl => Ok(Register8::Cl),
            Register::Dh => Ok(Register8::Dh),
            Register::Dl => Ok(Register8::Dl),
            _ => Err(Error::CannotCovertType(format!("{} is not a 8-bit register", register)))
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

    pub fn get32(&self, register: &Register32) -> u32 {
        use Register32::*;

        match register {
            Eax => self.get_eax(),
            Ebx => self.get_ebx(),
            Ecx => self.get_ecx(),
            Edx => self.get_edx(),
            Edi => self.edi,
            Esi => self.esi,
            Ebp => self.ebp,
            Esp => self.esp,
            // FIXME: likely a better way to structure this. Perhaps eflags -> u32.
            Eflags => unimplemented!(),
            Eip => self.eip,
        }
    }

    pub fn set32(&mut self, register: &Register32, value: u32) {
        use Register32::*;

        match register {
            Eax => self.set_eax(value),
            Ebx => self.set_ebx(value),
            Ecx => self.set_ecx(value),
            Edx => self.set_edx(value),
            Edi => self.edi = value,
            Esi => self.esi = value,
            Ebp => self.ebp = value,
            Esp => self.esp = value,
            // FIXME: likely a better way to structure this. Perhaps eflags -> u32.
            Eflags => unimplemented!(),
            Eip => self.eip = value,
        }
    }

    pub fn get16(&self, register: &Register16) -> u16 {
        use Register16::*;

        match register {
            Ax => self.get_ax(),
            Bx => self.get_bx(),
            Cx => self.get_cx(),
            Dx => self.get_dx(),
            Cs => self.cs,
            Ds => self.ds,
            Es => self.es,
            Fs => self.fs,
            Gs => self.gs,
            Ss => self.ss,
        }
    }

    pub fn set16(&mut self, register: &Register16, value: u16) {
        use Register16::*;

        match register {
            Ax => self.set_ax(value),
            Bx => self.set_bx(value),
            Cx => self.set_cx(value),
            Dx => self.set_dx(value),
            Cs => self.cs = value,
            Ds => self.ds = value,
            Es => self.es = value,
            Fs => self.fs = value,
            Gs => self.gs = value,
            Ss => self.ss = value,
        }
    }

    pub fn get8(&self, register: &Register8) -> u8 {
        use Register8::*;

        match register {
            Ah => self.get_ah(),
            Al => self.get_al(),
            Bh => self.get_bh(),
            Bl => self.get_bl(),
            Ch => self.get_ch(),
            Cl => self.get_cl(),
            Dh => self.get_dh(),
            Dl => self.get_dl(),
        }
    }

    pub fn set8(&mut self, register: &Register8, value: u8) {
        use Register8::*;

        match register {
            Ah => self.set_ah(value),
            Al => self.set_al(value),
            Bh => self.set_bh(value),
            Bl => self.set_bl(value),
            Ch => self.set_ch(value),
            Cl => self.set_cl(value),
            Dh => self.set_dh(value),
            Dl => self.set_dl(value),
        }
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
