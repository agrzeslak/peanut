use std::{fmt::Display, mem, u32};

use bitmaps::Bitmap;
use num_traits::{
    CheckedAdd, CheckedSub, FromPrimitive, PrimInt, Unsigned, WrappingAdd, WrappingSub,
};
use paste::paste;

use crate::{
    cpu::Operation,
    error::Error,
    instruction::{NasmStr, OperandType, Size},
    traits::{AsUnsigned, BitIndex, HighLowBytes32, MostSignificantBit, Signed},
};

pub enum CurrentPrivilegeLevel {
    CPL0,
    CPL1,
    CPL2,
    CPL3,
}

pub enum WithCarry {
    True,
    False,
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
/// indicates an overflow condition for signed-integer (two’s complement) arithmetic. I.e. it is
/// set when the most significant bit (sign bit) is changed by adding two numbers with the same
/// sign, or subtracting two numbers with opposite signs. Overflow cannot occur when the sign of
/// two addition operads are different, or the sign of two subtraction operands are the same. This
/// flag is meaningless/ignored for unsigned arithmetic.
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
#[derive(Clone, Debug)]
pub struct Eflags(Bitmap<32>);

macro_rules! eflags_accessors {
    ($field_name:ident, $bit:literal) => {
        paste! {
            pub fn [<get_ $field_name>](&self) -> bool {
                self.0.get($bit)
            }

            pub fn [<set_ $field_name>](&mut self, value: bool) {
                self.0.set($bit, value);
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

    /// Sets the carry flag based on whether the unsigned addition/subtraction generated a
    /// carry/borrow. For the purposes of computing the carry flag, we are only interested in
    /// unsigned integer addition, hence that bound has been added. If a signed integer was
    /// provided, an incorrect value would be produced.
    pub(crate) fn compute_carry_flag<T>(
        &mut self,
        a: T,
        b: T,
        operation: Operation,
        with_carry: WithCarry,
    ) where
        T: PrimInt + AsUnsigned,
    {
        let a = a.as_unsigned();
        let b = b.as_unsigned();
        let carried = match operation {
            Operation::Add => {
                match a.checked_add(&b) {
                    Some(n) => match with_carry {
                        // No zeroes means that adding the carry (1), will cause it to overflow.
                        WithCarry::True => n.count_zeros() == 0,
                        WithCarry::False => false,
                    },
                    None => true,
                }
            }
            Operation::Subtract => {
                match a.checked_sub(&b) {
                    Some(n) => match with_carry {
                        // No ones means that subtracting the carry (1), will cause it to underflow.
                        WithCarry::True => n.count_ones() == 0,
                        WithCarry::False => false,
                    },
                    None => true,
                }
            }
        };
        self.set_carry_flag(carried);
    }

    /// Sets the parity flag if the least significant byte of the result of the last operation has
    /// an even number of bits set to 1. Providing a value larger than u64 will panic, however this
    /// should never be the case.
    // TODO: Tests.
    pub(crate) fn compute_parity_flag<T: PrimInt + AsUnsigned + FromPrimitive>(
        &mut self,
        result: T,
    ) {
        let least_significant_byte = result.as_unsigned() & FromPrimitive::from_u8(0xFF).unwrap();
        self.set_parity_flag(least_significant_byte.count_ones() % 2 == 0);
    }

    /// Sets the overflow flag if the signed addition (two's complement) cannot fit within the
    /// number of bits. I.e. if two operands of the same sign are added, or two operands of
    /// opposite sign are subtracted and a result of different sign is produced.
    // TODO: Tests.
    pub(crate) fn compute_overflow_flag<T>(&mut self, a: T, b: T, result: T, operation: Operation)
    where
        T: PrimInt,
    {
        let overflowed = match operation {
            Operation::Add => a.sign() == b.sign() && result.sign() != a.sign(),
            Operation::Subtract => a.sign() != b.sign() && result.sign() != a.sign(),
        };
        self.set_overflow_flag(overflowed);
    }

    /// Sets the auxiliary carry flag if a carry or borrow is generated out of the 3rd bit.
    // FIXME: Addition is wrong. Should also generate a carry if one of the bits at index 3 are
    //        set, and there is a carry from a lower index into them.
    pub(crate) fn compute_auxiliary_carry_flag<T>(&mut self, a: T, b: T, operation: Operation)
    where
        T: PrimInt,
    {
        let carried = match operation {
            Operation::Add => a.bit_at_index(3) && b.bit_at_index(3),
            Operation::Subtract => todo!(),
        };
        self.set_auxiliary_carry_flag(carried);
    }

    /// Sets the zero flag if the result is 0.
    // TODO: Tests.
    pub(crate) fn compute_zero_flag<T: PrimInt>(&mut self, result: T) {
        self.set_zero_flag(result.count_ones() == 0);
    }

    /// Sets the sign flag to the most signifcant bit of the result.
    // TODO: Tests.
    pub(crate) fn compute_sign_flag<T: PrimInt>(&mut self, result: T) {
        self.set_sign_flag(result.most_significant_bit());
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
pub enum Register32 {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
}

impl Display for Register32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register32::*;
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

impl TryFrom<Register> for Register32 {
    type Error = Error;

    fn try_from(register: Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register32(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a general purpose (32-bit) register",
                register
            ))),
        }
    }
}

impl<'a> TryFrom<&'a Register> for &'a Register32 {
    type Error = Error;

    fn try_from(register: &'a Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register32(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a general purpose (32-bit) register",
                register
            ))),
        }
    }
}

impl TryFrom<&NasmStr<'_>> for Register32 {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Register32::*;
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

impl<'a> TryFrom<&'a OperandType> for &'a Register32 {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        <&Register>::try_from(operand_type)?.try_into()
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

impl TryFrom<Register> for Register16 {
    type Error = Error;

    fn try_from(register: Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register16(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 16-bit register",
                register
            ))),
        }
    }
}

impl<'a> TryFrom<&'a Register> for &'a Register16 {
    type Error = Error;

    fn try_from(register: &'a Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register16(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 16-bit register",
                register
            ))),
        }
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

impl<'a> TryFrom<&'a OperandType> for &'a Register16 {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        <&Register>::try_from(operand_type)?.try_into()
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

impl TryFrom<Register> for Register8 {
    type Error = Error;

    fn try_from(register: Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register8(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 8-bit register",
                register
            ))),
        }
    }
}

impl<'a> TryFrom<&'a Register> for &'a Register8 {
    type Error = Error;

    fn try_from(register: &'a Register) -> Result<Self, Self::Error> {
        match register {
            Register::Register8(register) => Ok(register),
            _ => Err(Error::CannotCovertType(format!(
                "{} is not a 8-bit register",
                register
            ))),
        }
    }
}

impl<'a> TryFrom<&'a OperandType> for &'a Register8 {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        <&Register>::try_from(operand_type)?.try_into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register {
    Register32(Register32),
    Register16(Register16),
    Register8(Register8),
}

impl Register {
    pub fn size(&self) -> Size {
        use Register::*;
        use Size::*;
        match self {
            Register32(_) => Dword,
            Register16(_) => Word,
            Register8(_) => Byte,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;
        match self {
            Register32(r) => r.fmt(f),
            Register16(r) => r.fmt(f),
            Register8(r) => r.fmt(f),
        }
    }
}

impl From<Register32> for Register {
    fn from(register: Register32) -> Self {
        Self::Register32(register)
    }
}

impl From<Register16> for Register {
    fn from(register: Register16) -> Self {
        Self::Register16(register)
    }
}

impl From<Register8> for Register {
    fn from(register: Register8) -> Self {
        Self::Register8(register)
    }
}

impl TryFrom<&NasmStr<'_>> for Register {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        match value.0.to_uppercase().as_str() {
            "EAX" => Ok(Register32::Eax.into()),
            "AX" => Ok(Register16::Ax.into()),
            "AH" => Ok(Register8::Ah.into()),
            "AL" => Ok(Register8::Al.into()),

            "EBX" => Ok(Register32::Ebx.into()),
            "BX" => Ok(Register16::Bx.into()),
            "BH" => Ok(Register8::Bh.into()),
            "BL" => Ok(Register8::Bl.into()),

            "ECX" => Ok(Register32::Ecx.into()),
            "CX" => Ok(Register16::Cx.into()),
            "CH" => Ok(Register8::Ch.into()),
            "CL" => Ok(Register8::Cl.into()),

            "EDX" => Ok(Register32::Edx.into()),
            "DX" => Ok(Register16::Dx.into()),
            "DH" => Ok(Register8::Dh.into()),
            "DL" => Ok(Register8::Dl.into()),

            "ESI" => Ok(Register32::Esi.into()),
            "SI" => Ok(Register16::Si.into()),

            "EDI" => Ok(Register32::Edi.into()),
            "DI" => Ok(Register16::Di.into()),

            "EBP" => Ok(Register32::Ebp.into()),
            "BP" => Ok(Register16::Bp.into()),

            "ESP" => Ok(Register32::Esp.into()),
            "SP" => Ok(Register16::Sp.into()),

            "CS" => Ok(Register16::Cs.into()),
            "DS" => Ok(Register16::Ds.into()),
            "SS" => Ok(Register16::Ss.into()),
            "ES" => Ok(Register16::Es.into()),
            "FS" => Ok(Register16::Fs.into()),
            "GS" => Ok(Register16::Gs.into()),

            _ => Err(Error::CannotParseInstruction(format!(
                "{} is not a valid register",
                value.0
            ))),
        }
    }
}

impl<'a> TryFrom<&'a OperandType> for &'a Register {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(_) => Err(Error::CannotCovertType(
                "an immediate was provided when a register was expected".into(),
            )),
            OperandType::Memory(_) => Err(Error::CannotCovertType(
                "a memory reference was provided when a register was expected".into(),
            )),
            OperandType::Register(register) => Ok(register),
        }
    }
}

#[derive(Clone, Debug, Default)]
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

    pub fn read32(&self, register: &Register32) -> u32 {
        use Register32::*;
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

    pub fn write32(&mut self, register: &Register32, value: u32) {
        use Register32::*;
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

    pub fn read16(&self, register: &Register16) -> u16 {
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

    pub fn write16(&mut self, register: &Register16, value: u16) {
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

    pub fn read8(&self, register: &Register8) -> u8 {
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

    pub fn write8(&mut self, register: &Register8, value: u8) {
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
