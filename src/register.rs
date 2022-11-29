use std::{fmt::Display, u32};

use bitmaps::Bitmap;
use paste::paste;

use crate::{
    error::Error,
    instruction::{NasmStr, Size},
};

trait HighLowBytes32 {
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

pub enum CurrentPrivilegeLevel {
    CPL0,
    CPL1,
    CPL2,
    CPL3,
}

/// Intel manual section 3.4.3 "EFLAGS Register".
/// - Status flags indicate the result of arithmetic instructions.
/// - System flags and the IOPL field control OS or executive operations. Application programs
///   should not modify them.
/// - Reserved bits and values: 1=1, 3=0, 5=0, 15=0, 22-31=0.
///
/// CF (Carry Flag), bit 0, status flag.
/// Set if an arithmetic operation generates a carry or a borrow out of the most-significant
/// bit of the result; cleared otherwise. This flag indicates an overflow condition for
/// unsigned-integer arithmetic. It is also used in multiple-precision arithmetic.
///
/// PF (Parity Flag), bit 2, status flag.
/// Set if the least-significant byte of the result contains an even number of 1 bits; cleared
/// otherwise.
///
/// AF (Auxiliary Carry Flag), bit 4, status flag.
/// Set if an arithmetic operation generates a carry or a borrow out of bit 3 of the result;
/// cleared otherwise. This flag is used in binary-coded decimal (BCD) arithmetic.
///
/// ZF (Zero Flag), bit 6, status flag.
/// Set if the result is zero; cleared otherwise.
///
/// SF (Sign Flag), bit 7, status flag.
/// Set equal to the most-significant bit of the result, which is the sign bit of a signed
/// integer. (0 indicates a positive value and 1 indicates a negative value.)
///
/// TF (Trap Flag), bit 8, system flag.
/// Set to enable single-step mode for debugging; clear to disable single-step mode.
///
/// IF (Interrupt Enable Flag), bit 9, system flag.
/// Controls the response of the processor to maskable interrupt requests. Set to respond to
/// maskable interrupts; cleared to inhibit maskable interrupts.
///
/// DF (Direction Flag), bit 10, control flag.
/// Controls string instructions (MOVS, CMPS, SCAS, LODS, and STOS). Setting the DF flag causes
/// the string instructions to auto-decrement (to process strings from high addresses to low
/// addresses). Clearing the DF flag causes the string instructions to auto-increment (process
/// strings from low addresses to high addresses). The STD and CLD instructions set and clear
/// the DF flag, respectively.
///
/// OF (Overflow Flag), bit 11, status flag.
/// Set if the integer result is too large a positive number or too small a negative number
/// (excluding the sign-bit) to fit in the destination operand; cleared otherwise. This flag
/// indicates an overflow condition for signed-integer (two’s complement) arithmetic.
///
/// IOPL (I/O Privilege Level Field), bits 12 and 13, system flag.
/// Indicates the I/O privilege level of the currently running program or task. The current
/// privilege level (CPL) of the currently running program or task must be less than or equal
/// to the I/O privilege level to access the I/O address space. The POPF and IRET instructions
/// can modify this field only when operating at a CPL of 0.
///
/// NT (Nested Task Flag), bit 14, system flag.
/// Controls the chaining of interrupted and called tasks. Set when the current task is linked
/// to the previously executed task; cleared when the current task is not linked to another
/// task.
///
/// RF (Resume Flag), bit 16, system flag.
/// Controls the processor’s response to debug exceptions.
///
/// VM (Virtual-8086 Mode Flag), bit 17, system flag.
/// Set to enable virtual-8086 mode; clear to return to protected mode without virtual-8086
/// mode semantics.
///
/// AC (Alignment Check (or Access Control) Flag), bit 18, system flag.
/// If the AM bit is set in the CR0 register, alignment checking of user-mode data accesses is
/// enabled if and only if this flag is 1. If the SMAP bit is set in the CR4 register, explicit
/// supervisor-mode data accesses to user-mode pages are allowed if and only if this bit is 1.
/// See Section 4.6, “Access Rights,” in the Intel® 64 and IA-32 Architectures Software
/// Developer’s Manual, Volume 3A.
///
/// VIF (Virtual Interrupt Flag), bit 19, system flag.
/// Virtual image of the IF flag. Used in conjunction with the VIP flag. (To use this flag and
/// the VIP flag the virtual mode extensions are enabled by setting the VME flag in control
/// register CR4).
///
/// VIP (Virtual Interrupt Pending Flag), bit 20, system flag.
/// Set to indicate that an interrupt is pending; clear when no interrupt is pending. (Software
/// sets and clears this flag; the processor only reads it.) Used in conjunction with the VIF
/// flag.
///
/// ID (Identification Flag), bit 21, system flag.
/// The ability of a program to set or clear this flag indicates support for the CPUID
/// instruction.
pub struct Eflags(Bitmap<32>);

macro_rules! eflags_accessors {
    ($field_name:ident, $bit:literal) => {
        paste! {
            pub fn [<get_ $field_name>](&self) -> bool {
                self.0.get($bit)
            }

            pub fn [<set_ $field_name>](&mut self, value: bool) {
                self.0.set($bit, true);
            }
        }
    };
}

impl Eflags {
    eflags_accessors!(carry_flag, 0);
    eflags_accessors!(parity_flag, 2);
    eflags_accessors!(auxiliary_carry_flag, 4);
    eflags_accessors!(zero_flag, 6);
    eflags_accessors!(sign_flag, 7);
    eflags_accessors!(trap_flag, 8);
    eflags_accessors!(interrupt_enable_flag, 9);
    eflags_accessors!(direction_flag, 10);
    eflags_accessors!(overflow_flag, 11);
    eflags_accessors!(nested_task, 14);
    eflags_accessors!(resume_flag, 16);
    eflags_accessors!(virtual_8086_mode, 17);
    eflags_accessors!(alignment_check, 18);
    eflags_accessors!(virtual_interrupt_flag, 19);
    eflags_accessors!(virtual_interrupt_pending_flag, 20);
    eflags_accessors!(identification_flag, 21);

    /// Sets the parity flag if the least significant byte of the result of the last operation has
    /// an even number of bits set to 1.
    pub fn compute_parity_flag(&mut self, least_significant_byte: u8) {
        self.set_parity_flag(least_significant_byte.count_ones() % 2 == 0);
    }

    pub fn get_iopl(&self) -> CurrentPrivilegeLevel {
        let first_bit = self.0.get(12);
        let second_bit = self.0.get(13);
        // TODO: Verify that these bits correspond to the correct privilege levels.
        match (second_bit, first_bit) {
            (false, false) => CurrentPrivilegeLevel::CPL0,
            (false, true) => CurrentPrivilegeLevel::CPL1,
            (true, false) => CurrentPrivilegeLevel::CPL2,
            (true, true) => CurrentPrivilegeLevel::CPL3,
        }
    }

    pub fn set_iopl(&mut self, cpl: CurrentPrivilegeLevel) {
        // TODO: Verify that these bits correspond to the correct privilege levels.
        let (second_bit, first_bit) = match cpl {
            CurrentPrivilegeLevel::CPL0 => (false, false),
            CurrentPrivilegeLevel::CPL1 => (false, true),
            CurrentPrivilegeLevel::CPL2 => (true, false),
            CurrentPrivilegeLevel::CPL3 => (true, true),
        };
        self.0.set(12, first_bit);
        self.0.set(13, second_bit);
    }
}

impl Default for Eflags {
    fn default() -> Self {
        let mut bitmap = Bitmap::new();
        // Bit 1 is the only reserved bit whose value is 1.
        bitmap.set(1, true);
        Self(bitmap)
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
    Si,
    Di,
    Bp,
    Sp,
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
            Si => "SI",
            Di => "DI",
            Bp => "BP",
            Sp => "SP",
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
            "SI" => Ok(Si),
            "DI" => Ok(Di),
            "BP" => Ok(Bp),
            "SP" => Ok(Sp),
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

    Esi,
    Si,

    Edi,
    Di,

    Ebp,
    Bp,

    Esp,
    Sp,

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
            Eax | Ebx | Ecx | Edx | Esi | Edi | Ebp | Esp | Eflags | Eip => Dword,
            Ax | Bx | Cx | Dx | Si | Di | Bp | Sp | Cs | Ds | Es | Fs | Gs | Ss => Word,
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
            Si => "SI",

            Edi => "EDI",
            Di => "DI",

            Ebp => "EBP",
            Bp => "BP",

            Esp => "ESP",
            Sp => "SP",

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
            Si => Self::Si,
            Di => Self::Di,
            Bp => Self::Bp,
            Sp => Self::Sp,
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
            "SI" => Ok(Si),

            "EDI" => Ok(Edi),
            "DI" => Ok(Di),

            "EBP" => Ok(Ebp),
            "BP" => Ok(Bp),

            "ESP" => Ok(Esp),
            "SP" => Ok(Sp),

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
            Si => Ok(Self::Si),
            Di => Ok(Self::Di),
            Bp => Ok(Self::Bp),
            Sp => Ok(Self::Sp),
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
    pub(crate) eax: u32,
    pub(crate) ebx: u32,
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
    pub(crate) esi: u32,
    pub(crate) edi: u32,
    pub(crate) ebp: u32,
    pub(crate) esp: u32,
    pub(crate) cs: u16,
    pub(crate) ds: u16,
    pub(crate) es: u16,
    pub(crate) fs: u16,
    pub(crate) gs: u16,
    pub(crate) ss: u16,
    pub(crate) eflags: Eflags,

    /// Intel manual section 3.5 "INSTRUCTION POINTER".
    /// Contains offset in current code segment for next instruction to be executed. Cannot be
    /// accessed directly by software. IA-32 processors prefetch instrucitons, meaning that the
    /// address read from the bus during an instruction load does not match the EIP register.
    eip: u32,
}

macro_rules! abcd_register_accessors {
    ($register_letter:ident) => {
        paste! {
            pub fn [<get_e $register_letter x>](&self) -> u32 {
                self.[<e $register_letter x>]
            }

            pub fn [<set_e $register_letter x>](&mut self, value: u32) {
                self.[<e $register_letter x>] = value;
            }

            pub fn [<get_ $register_letter x>](&self) -> u16 {
                self.[<e $register_letter x>].get_low_16()
            }

            pub fn [<set_ $register_letter x>](&mut self, value: u16) {
                self.[<e $register_letter x>].set_low_16(value)
            }

            pub fn [<get_ $register_letter h>](&self) -> u8 {
                self.[<e $register_letter x>].get_high_8()
            }

            pub fn [<set_ $register_letter h>](&mut self, value: u8) {
                self.[<e $register_letter x>].set_high_8(value);
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
    abcd_register_accessors!(a);
    abcd_register_accessors!(b);
    abcd_register_accessors!(c);
    abcd_register_accessors!(d);

    pub fn get_si(&self) -> u16 {
        self.esi.get_low_16()
    }

    pub fn set_si(&mut self, value: u16) {
        self.esi.set_low_16(value);
    }

    pub fn get_di(&self) -> u16 {
        self.edi.get_low_16()
    }

    pub fn set_di(&mut self, value: u16) {
        self.edi.set_low_16(value);
    }

    pub fn get_bp(&self) -> u16 {
        self.ebp.get_low_16()
    }

    pub fn set_bp(&mut self, value: u16) {
        self.ebp.set_low_16(value);
    }

    pub fn get_sp(&self) -> u16 {
        self.esp.get_low_16()
    }

    pub fn set_sp(&mut self, value: u16) {
        self.esp.set_low_16(value);
    }

    pub fn get32(&self, register: &GeneralPurposeRegister) -> u32 {
        use GeneralPurposeRegister::*;
        match register {
            Eax => self.get_eax(),
            Ebx => self.get_ebx(),
            Ecx => self.get_ecx(),
            Edx => self.get_edx(),
            Esi => self.esi,
            Edi => self.edi,
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
            Esi => self.esi = value,
            Edi => self.edi = value,
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
            Si => self.get_si(),
            Di => self.get_di(),
            Bp => self.get_bp(),
            Sp => self.get_sp(),
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
            Si => self.set_si(value),
            Di => self.set_di(value),
            Bp => self.set_bp(value),
            Sp => self.set_bp(value),
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
    /// `e $register_letter x`, i.e. `eax`.
    macro_rules! test_abcd_register_accessors {
        ($register_letter:ident) => {
            paste! {
                let mut registers = Registers::default();
                registers.[<set_e $register_letter x>](0xdeadc0de);
                assert_eq!(registers.[<get_e $register_letter x>](), 0xdeadc0de);
                assert_eq!(registers.[<get_ $register_letter x>](), 0xc0de);
                assert_eq!(registers.[<get_ $register_letter h>](), 0xc0);
                assert_eq!(registers.[<get_ $register_letter l>](), 0xde);

                registers.[<set_ $register_letter x>](0xb33f);
                assert_eq!(registers.[<get_e $register_letter x>](), 0xdeadb33f as u32);
                assert_eq!(registers.[<get_ $register_letter x>](), 0xb33f);
                assert_eq!(registers.[<get_ $register_letter h>](), 0xb3);
                assert_eq!(registers.[<get_ $register_letter l>](), 0x3f);
            }
        };
    }

    #[test]
    fn eax_get_and_set() {
        test_abcd_register_accessors!(a);
    }

    #[test]
    fn ebx_get_and_set() {
        test_abcd_register_accessors!(b);
    }

    #[test]
    fn ecx_get_and_set() {
        test_abcd_register_accessors!(c);
    }

    #[test]
    fn edx_get_and_set() {
        test_abcd_register_accessors!(d);
    }
}
