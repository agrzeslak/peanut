use std::{fmt::Display, u32};

use bitmaps::Bitmap;
use paste::paste;

use crate::{
    error::Error,
    instruction::{NasmStr, Size},
};

trait HighLowBytes32 {
    fn get_high_16(&self) -> u16;
    fn set_high_16(&mut self, value: u16);
    fn get_high_8(&self) -> u8;
    fn set_high_8(&mut self, value: u8);
    fn get_low_8(&self) -> u8;
    fn set_low_8(&mut self, value: u8);
}

impl HighLowBytes32 for u32 {
    fn get_high_16(&self) -> u16 {
        (*self >> 16) as u16
    }

    fn set_high_16(&mut self, value: u16) {
        *self &= 0x0000ffff;
        *self |= (value as u32) << 16
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
pub enum GeneralPurposeRegister {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
}

impl Display for GeneralPurposeRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GeneralPurposeRegister::*;
        let register = match self {
            Eax => "EAX",
            Ebx => "EBX",
            Ecx => "ECX",
            Edx => "EDX",
            Esi => "ESI",
            Edi => "EDI",
            Ebp => "EBP",
            Esp => "ESP",
        };

        write!(f, "{register}")
    }
}

impl TryFrom<&NasmStr<'_>> for GeneralPurposeRegister {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use GeneralPurposeRegister::*;
        match value.0.to_uppercase().as_str() {
            "EAX" => Ok(Eax),
            "EBX" => Ok(Ebx),
            "ECX" => Ok(Ecx),
            "EDX" => Ok(Edx),
            "ESI" => Ok(Esi),
            "EDI" => Ok(Edi),
            "EBP" => Ok(Ebp),
            "ESP" => Ok(Esp),
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
    Ss,
    Es,
    Fs,
    Gs,
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
            Ss => "SS",
            Es => "ES",
            Fs => "FS",
            Gs => "GS",
        };

        write!(f, "{register}")
    }
}

impl TryFrom<&NasmStr<'_>> for Register16 {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register16::*;
        match value.0.to_uppercase().as_str() {
            "AX" => Ok(Ax),
            "BX" => Ok(Bx),
            "CX" => Ok(Cx),
            "DX" => Ok(Dx),
            "CS" => Ok(Cs),
            "DS" => Ok(Ds),
            "SS" => Ok(Ss),
            "ES" => Ok(Es),
            "FS" => Ok(Fs),
            "GS" => Ok(Gs),
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
        match value.0.to_uppercase().as_str() {
            "AH" => Ok(Ah),
            "AL" => Ok(Al),
            "BH" => Ok(Bh),
            "BL" => Ok(Bl),
            "CH" => Ok(Ch),
            "CL" => Ok(Cl),
            "DH" => Ok(Dh),
            "DL" => Ok(Dl),
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

    Cs,
    Ds,
    Es,
    Fs,
    Gs,
    Ss,

    Eflags,
    Eip,
}

impl Register {
    pub fn size(&self) -> Size {
        use Register::*;
        use Size::*;
        match self {
            Eax | Ebx | Ecx | Edx |  Esi | Edi | Ebp | Esp | Eflags | Eip => Dword,
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

            Esi => "ESI",
            Edi => "EDI",
            Ebp => "EBP",
            Esp => "ESP",

            Cs => "CS",
            Ds => "DS",
            Ss => "SS",
            Es => "ES",
            Fs => "FS",
            Gs => "GS",

            Eflags => "EFLAGS",
            Eip => "EIP",
        };

        write!(f, "{register}")
    }
}

impl From<&GeneralPurposeRegister> for Register {
    fn from(register: &GeneralPurposeRegister) -> Self {
        use GeneralPurposeRegister::*;
        match register {
            Eax => Self::Eax,
            Ebx => Self::Ebx,
            Ecx => Self::Ecx,
            Edx => Self::Edx,
            Esi => Self::Esi,
            Edi => Self::Edi,
            Ebp => Self::Ebp,
            Esp => Self::Esp,
        }
    }
}

impl From<&Register16> for Register {
    fn from(register: &Register16) -> Self {
        use Register16::*;
        match register {
            Ax => Self::Ax,
            Bx => Self::Bx,
            Cx => Self::Cx,
            Dx => Self::Dx,
            Cs => Self::Cs,
            Ds => Self::Ds,
            Ss => Self::Ss,
            Es => Self::Es,
            Fs => Self::Fs,
            Gs => Self::Gs,
        }
    }
}

impl From<&Register8> for Register {
    fn from(register: &Register8) -> Self {
        use Register8::*;
        match register {
            Ah => Self::Ah,
            Al => Self::Al,
            Bh => Self::Bh,
            Bl => Self::Bl,
            Ch => Self::Ch,
            Cl => Self::Cl,
            Dh => Self::Dh,
            Dl => Self::Dl,
        }
    }
}

impl TryFrom<&NasmStr<'_>> for Register {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register::*;
        match value.0.to_uppercase().as_str() {
            "EAX" => Ok(Eax),
            "AX" => Ok(Ax),
            "AH" => Ok(Ah),
            "AL" => Ok(Al),

            "EBX" => Ok(Ebx),
            "BX" => Ok(Bx),
            "BH" => Ok(Bh),
            "BL" => Ok(Bl),

            "ECX" => Ok(Ecx),
            "CX" => Ok(Cx),
            "CH" => Ok(Ch),
            "CL" => Ok(Cl),

            "EDX" => Ok(Edx),
            "DX" => Ok(Dx),
            "DH" => Ok(Dh),
            "DL" => Ok(Dl),

            "ESI" => Ok(Esi),
            "EDI" => Ok(Edi),
            "EBP" => Ok(Ebp),
            "ESP" => Ok(Esp),

            "CS" => Ok(Cs),
            "DS" => Ok(Ds),
            "SS" => Ok(Ss),
            "ES" => Ok(Es),
            "FS" => Ok(Fs),
            "GS" => Ok(Gs),

            "EFLAGS" => Ok(Eflags),
            "EIP" => Ok(Eip),

            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid register",
                value.0
            ))),
        }
    }
}

impl TryFrom<&Register> for GeneralPurposeRegister {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        use Register::*;
        match register {
            Eax => Ok(Self::Eax),
            Ebx => Ok(Self::Ebx),
            Ecx => Ok(Self::Ecx),
            Edx => Ok(Self::Edx),
            Esi => Ok(Self::Esi),
            Edi => Ok(Self::Edi),
            Ebp => Ok(Self::Ebp),
            Esp => Ok(Self::Esp),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a general purpose register",
                register
            ))),
        }
    }
}

impl TryFrom<&Register> for Register16 {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        use Register::*;
        match register {
            Ax => Ok(Self::Ax),
            Bx => Ok(Self::Bx),
            Cx => Ok(Self::Cx),
            Dx => Ok(Self::Dx),
            Cs => Ok(Self::Cs),
            Ds => Ok(Self::Ds),
            Ss => Ok(Self::Ss),
            Es => Ok(Self::Es),
            Fs => Ok(Self::Fs),
            Gs => Ok(Self::Gs),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 16-bit register",
                register
            ))),
        }
    }
}

impl TryFrom<&Register> for Register8 {
    type Error = Error;

    fn try_from(register: &Register) -> Result<Self, Self::Error> {
        use Register::*;
        match register {
            Ah => Ok(Self::Ah),
            Al => Ok(Self::Al),
            Bh => Ok(Self::Bh),
            Bl => Ok(Self::Bl),
            Ch => Ok(Self::Ch),
            Cl => Ok(Self::Cl),
            Dh => Ok(Self::Dh),
            Dl => Ok(Self::Dl),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 8-bit register",
                register
            ))),
        }
    }
}

#[derive(Default)]
pub struct Registers {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    esp: u32,
    cs: u16,
    ds: u16,
    es: u16,
    fs: u16,
    gs: u16,
    ss: u16,
    eflags: Eflags,
    eip: u32,
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
                self.[<e $register_letter x>].get_high_16()
            }

            pub fn [<set_ $register_letter x>](&mut self, value: u16) {
                self.[<e $register_letter x>].set_high_16(value)
            }

            pub fn [<get_ $register_letter h>](&self) -> u8 {
                self.[<e $register_letter x>].get_low_8()
            }

            pub fn [<set_ $register_letter h>](&mut self, value: u8) {
                self.[<e $register_letter x>].set_low_8(value);
            }

            pub fn [<get_ $register_letter l>](&self) -> u8 {
                self.[<e $register_letter x>].get_low_8()
            }

            pub fn [<set_ $register_letter l>](&mut self, value: u8) {
                self.[<e $register_letter x>].set_low_8(value);
            }
        }
    };
}
impl Registers {
    create_general_register_accessors!(a);
    create_general_register_accessors!(b);
    create_general_register_accessors!(c);
    create_general_register_accessors!(d);

    pub fn get32(&self, register: &GeneralPurposeRegister) -> u32 {
        use GeneralPurposeRegister::*;
        match register {
            Eax => self.get_eax(),
            Ebx => self.get_ebx(),
            Ecx => self.get_ecx(),
            Edx => self.get_edx(),
            Edi => self.edi,
            Esi => self.esi,
            Ebp => self.ebp,
            Esp => self.esp,
        }
    }

    pub fn set32(&mut self, register: &GeneralPurposeRegister, value: u32) {
        use GeneralPurposeRegister::*;
        match register {
            Eax => self.set_eax(value),
            Ebx => self.set_ebx(value),
            Ecx => self.set_ecx(value),
            Edx => self.set_edx(value),
            Edi => self.edi = value,
            Esi => self.esi = value,
            Ebp => self.ebp = value,
            Esp => self.esp = value,
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
