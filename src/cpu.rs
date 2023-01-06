use std::ops::{BitAnd, BitOr};

use num_traits::{
    CheckedAdd, CheckedSub, FromPrimitive, PrimInt, Unsigned, WrappingAdd, WrappingSub,
};

use crate::{
    instruction::{
        unwrap_operands, Immediate, Instruction, RegisterOrMemory16, RegisterOrMemory32,
        RegisterOrMemory8,
    },
    register::{Register16, Register32, Register8, Registers, WithCarry},
    traits::AsUnsigned,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
}

#[derive(Clone, Debug, Default)]
pub struct Cpu {
    pub(crate) registers: Registers,
}

impl Cpu {
    /// Performs wrapping addition, adding the carry flag if required, and setting flags which can
    /// only be known by observing the addition. These are OF, and AF. Other flags are not set and
    /// should be set separately.
    fn wrapping_add<T>(&mut self, lhs: T, rhs: T, with_carry: WithCarry) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive + AsUnsigned,
    {
        let result = lhs.wrapping_add(&rhs);
        if let WithCarry::True = with_carry {
            let carry = self.registers.eflags.get_carry_flag() as u8;
            let carry = FromPrimitive::from_u8(carry).unwrap();
            result.wrapping_add(&carry);
        }

        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Add);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Add);

        result
    }

    /// Performs wrapping subtraction, settings flags which can only be known by observing the
    /// subtraction. These are OF, and AF.
    fn wrapping_sub<T>(&mut self, lhs: T, rhs: T, with_carry: WithCarry) -> T
    where
        T: PrimInt + WrappingSub + FromPrimitive + AsUnsigned,
    {
        let result = lhs.wrapping_sub(&rhs);
        if let WithCarry::True = with_carry {
            let carry = self.registers.eflags.get_carry_flag() as u8;
            let carry = FromPrimitive::from_u8(carry).unwrap();
            result.wrapping_sub(&carry);
        }

        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Subtract);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Subtract);

        result
    }

    /// Add the two operands and carry together, wrapping if an overflow occurs, and set the
    /// OF, SF, ZF, AF, CF, and PF flags according to the result.
    fn adc<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive + AsUnsigned,
    {
        let result = self.wrapping_add(lhs, rhs, WithCarry::True);
        self.registers.eflags.compute_parity_flag(result);
        self.registers
            .eflags
            .compute_carry_flag(lhs, rhs, Operation::Add, WithCarry::True);
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_sign_flag(result);
        result
    }

    pub(crate) fn adc_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        let result = self.adc(self.registers.get_al(), imm8.parsed() as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn adc_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        let result = self.adc(self.registers.get_ax(), imm16.parsed() as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn adc_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        let result = self.adc(self.registers.get_eax(), imm32.parsed() as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn adc_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        let result = self.adc(self.registers.read8(reg8), rm8.read8(self));
        self.registers.write8(&reg8, result);
    }

    pub(crate) fn adc_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        let result = self.adc(self.registers.read16(reg16), rm16.read16(self));
        self.registers.write16(&reg16, result);
    }

    pub(crate) fn adc_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        let result = self.adc(self.registers.read32(reg32), rm32.read32(self));
        self.registers.write32(&reg32, result);
    }

    pub(crate) fn adc_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        let result = self.adc(rm8.read8(self), self.registers.read8(reg8));
        rm8.write8(self, result);
    }

    pub(crate) fn adc_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        let result = self.adc(rm16.read16(self), self.registers.read16(reg16));
        rm16.write16(self, result);
    }

    pub(crate) fn adc_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        let result = self.adc(rm32.read32(self), self.registers.read32(reg32));
        rm32.write32(self, result);
    }

    /// Add the two operands together, wrapping if an overflow occurs, and set the OF, SF, ZF, AF,
    /// CF, and PF flags according to the result.
    fn add<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive + AsUnsigned,
    {
        let result = self.wrapping_add(lhs, rhs, WithCarry::False);
        self.registers.eflags.compute_parity_flag(result);
        self.registers.eflags.compute_carry_flag(
            lhs,
            rhs,
            Operation::Add,
            WithCarry::False,
        );
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_sign_flag(result);
        result
    }

    pub(crate) fn add_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        let result = self.add(self.registers.get_al(), imm8.parsed() as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn add_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        let result = self.add(self.registers.get_ax(), imm16.parsed() as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn add_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        let result = self.add(self.registers.get_eax(), imm32.parsed() as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn add_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        let result = self.add(self.registers.read8(reg8), rm8.read8(self));
        self.registers.write8(&reg8, result);
    }

    pub(crate) fn add_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        let result = self.add(self.registers.read16(reg16), rm16.read16(self));
        self.registers.write16(&reg16, result);
    }

    pub(crate) fn add_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        let result = self.add(self.registers.read32(reg32), rm32.read32(self));
        self.registers.write32(&reg32, result);
    }

    pub(crate) fn add_rm8_imm8(&mut self, instruction: &Instruction) {
        let (rm8, imm8) = unwrap_operands!(instruction, RegisterOrMemory8, &Immediate);
        let result = self.add(rm8.read8(&self), imm8.parsed() as u8);
        rm8.write8(self, result);
    }

    pub(crate) fn add_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        let result = self.add(rm8.read8(self), self.registers.read8(reg8));
        rm8.write8(self, result);
    }

    pub(crate) fn add_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        let result = self.add(rm16.read16(self), self.registers.read16(reg16));
        rm16.write16(self, result);
    }

    pub(crate) fn add_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        let result = self.add(rm32.read32(self), self.registers.read32(reg32));
        rm32.write32(self, result);
    }

    /// Performs a bitwise AND operation. Clears the OF and CF flags, and sets the SF, ZF, and PF
    /// flags depending on the result. The state of the AF flag is undefined.
    /// TODO: Tests.
    fn and<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + BitAnd<Output = T> + AsUnsigned + FromPrimitive,
    {
        let result = lhs & rhs;
        self.registers.eflags.set_overflow_flag(false);
        self.registers.eflags.set_carry_flag(false);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_parity_flag(result);
        result
    }

    pub(crate) fn and_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        let result = self.and(self.registers.get_al(), imm8.parsed() as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn and_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        let result = self.and(self.registers.get_ax(), imm16.parsed() as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn and_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        let result = self.and(self.registers.get_eax(), imm32.parsed() as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn and_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        let result = self.and(self.registers.read8(reg8), rm8.read8(self));
        self.registers.write8(reg8, result);
    }

    pub(crate) fn and_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        let result = self.and(self.registers.read16(reg16), rm16.read16(self));
        self.registers.write16(reg16, result);
    }

    pub(crate) fn and_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        let result = self.and(self.registers.read32(reg32), rm32.read32(self));
        self.registers.write32(reg32, result);
        todo!()
    }

    pub(crate) fn and_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        let result = self.and(rm8.read8(self), self.registers.read8(reg8));
        rm8.write8(self, result);
    }

    pub(crate) fn and_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        let result = self.and(rm16.read16(self), self.registers.read16(reg16));
        rm16.write16(self, result);
    }

    pub(crate) fn and_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        let result = self.and(rm32.read32(self), self.registers.read32(reg32));
        rm32.write32(self, result);
    }

    pub(crate) fn es(&mut self, instruction: &Instruction) {
        todo!()
    }

    pub(crate) fn daa(&mut self, instruction: &Instruction) {
        todo!()
    }

    /// Performs a bitwise inclusive OR operation. The OF and CF flags are cleared, and the SF, ZF,
    /// and PF flags are set according to the result. The AF flag is undefined.
    /// TODO: Tests.
    fn or<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + BitOr<T> + AsUnsigned + FromPrimitive,
    {
        let result = lhs | rhs;
        self.registers.eflags.set_overflow_flag(false);
        self.registers.eflags.set_carry_flag(false);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_parity_flag(result);
        result
    }
    pub(crate) fn or_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        let result = self.or(self.registers.get_al(), imm8.parsed() as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn or_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        let result = self.or(self.registers.get_ax(), imm16.parsed() as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn or_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        let result = self.or(self.registers.get_eax(), imm32.parsed() as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn or_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        let result = self.or(self.registers.read8(reg8), rm8.read8(self));
        self.registers.write8(reg8, result);
    }

    pub(crate) fn or_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        let result = self.or(self.registers.read16(reg16), rm16.read16(self));
        self.registers.write16(reg16, result);
    }

    pub(crate) fn or_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        let result = self.or(self.registers.read32(reg32), rm32.read32(self));
        self.registers.write32(reg32, result);
    }

    pub(crate) fn or_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        let result = self.or(rm8.read8(self), self.registers.read8(reg8));
        rm8.write8(self, result);
    }

    pub(crate) fn or_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        let result = self.or(rm16.read16(self), self.registers.read16(reg16));
        rm16.write16(self, result);
    }

    pub(crate) fn or_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        let result = self.or(rm32.read32(self), self.registers.read32(reg32));
        rm32.write32(self, result);
    }

    pub(crate) fn pop_ds(&mut self, instruction: &Instruction) {
        let _ds = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn pop_es(&mut self, instruction: &Instruction) {
        let _es = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn pop_ss(&mut self, instruction: &Instruction) {
        let _ss = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn push_cs(&mut self, instruction: &Instruction) {
        let _cs = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn push_ds(&mut self, instruction: &Instruction) {
        let _ds = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn push_es(&mut self, instruction: &Instruction) {
        let _es = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    pub(crate) fn push_ss(&mut self, instruction: &Instruction) {
        let _ss = unwrap_operands!(instruction, &Register16);
        todo!()
    }

    /// Integer subtraction with borrow. Adds the source and the carry flag, and subtracts the
    /// result from the destination. Sets the OF, SF, ZF, AF, PF, and CF flags according to the
    /// result.
    // TODO: Test
    fn sbb<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingSub + AsUnsigned + FromPrimitive,
    {
        let result = self.wrapping_sub(lhs, rhs, WithCarry::True);
        self.registers.eflags.compute_parity_flag(result);
        self.registers.eflags.compute_carry_flag(
            lhs,
            rhs,
            Operation::Subtract,
            WithCarry::True,
        );
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_sign_flag(result);
        result
    }

    pub(crate) fn sbb_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        todo!()
    }

    pub(crate) fn sbb_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        todo!()
    }

    pub(crate) fn sbb_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        todo!()
    }

    pub(crate) fn sbb_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        todo!()
    }

    pub(crate) fn sbb_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        todo!()
    }

    pub(crate) fn sbb_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        todo!()
    }

    pub(crate) fn sbb_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        todo!()
    }

    pub(crate) fn sbb_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        todo!()
    }

    pub(crate) fn sbb_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        todo!()
    }

    /// Integer subtraction. Adds the source and the carry flag, and subtracts the result from the
    /// destination. Sets the OF, SF, ZF, AF, PF, and CF flags according to the result.
    fn sub<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingSub + AsUnsigned + FromPrimitive,
    {
        let result = self.wrapping_sub(lhs, rhs, WithCarry::False);
        self.registers.eflags.compute_parity_flag(result);
        self.registers.eflags.compute_carry_flag(
            lhs,
            rhs,
            Operation::Subtract,
            WithCarry::False,
        );
        self.registers.eflags.compute_zero_flag(result);
        self.registers.eflags.compute_sign_flag(result);
        result
    }


    pub(crate) fn sub_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        todo!()
    }

    pub(crate) fn sub_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        todo!()
    }

    pub(crate) fn sub_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        todo!()
    }

    pub(crate) fn sub_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        todo!()
    }

    pub(crate) fn sub_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        todo!()
    }

    pub(crate) fn sub_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        todo!()
    }

    pub(crate) fn sub_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        todo!()
    }

    pub(crate) fn sub_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        todo!()
    }

    pub(crate) fn sub_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eflags {
        (@ $cpu:ident, OF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_overflow_flag(), $expected, "OF is incorrect")
        };
        (@ $cpu:ident, SF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_sign_flag(), $expected, "SF is incorrect")
        };
        (@ $cpu:ident, ZF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_zero_flag(), $expected, "ZF is incorrect")
        };
        (@ $cpu:ident, CF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_carry_flag(), $expected, "CF is incorrect")
        };
        ($cpu:ident, $($flag:ident=$expected:literal),+) => {
            $(assert_eflags!(@ $cpu, $flag=$expected));+
        };
    }

    // https://stackoverflow.com/questions/8965923/carry-overflow-subtraction-in-x86#8982549
    //       A                   B                   A + B              Flags
    // ---------------     ----------------    ---------------      -----------------
    // h  |  ud  |   d   | h  |  ud  |   d   | h  |  ud  |   d   | OF | SF | ZF | CF
    // ---+------+-------+----+------+-------+----+------+-------+----+----+----+---
    // 7F | 127  |  127  | 0  |  0   |   0   | 7F | 127  |  127  | 0  | 0  | 0  | 0
    // FF | 255  |  -1   | 7F | 127  |  127  | 7E | 126  |  126  | 0  | 0  | 0  | 1
    // 0  |  0   |   0   | 0  |  0   |   0   | 0  |  0   |   0   | 0  | 0  | 1  | 0
    // FF | 255  |  -1   | 1  |  1   |   1   | 0  |  0   |   0   | 0  | 0  | 1  | 1
    // FF | 255  |  -1   | 0  |  0   |   0   | FF | 255  |  -1   | 0  | 1  | 0  | 0
    // FF | 255  |  -1   | FF | 255  |  -1   | FE | 254  |  -2   | 0  | 1  | 0  | 1
    // FF | 255  |  -1   | 80 | 128  | -128  | 7F | 127  |  127  | 1  | 0  | 0  | 1
    // 80 | 128  | -128  | 80 | 128  | -128  | 0  |  0   |   0   | 1  | 0  | 1  | 1
    // 7F | 127  |  127  | 7F | 127  |  127  | FE | 254  |  -2   | 1  | 1  | 0  | 0
    #[test]
    fn add() {
        let mut cpu = Cpu::default();

        // Decimal
        assert_eq!(cpu.add(127_i8, 0_i8), 127_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.add(-1_i8, 127_i8), 126_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(0_i8, 0_i8), 0_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.add(-1_i8, 1_i8), 0_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(-1_i8, 0_i8), -1_i8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.add(-1_i8, -1_i8), -2_i8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.add(-1_i8, -128_i8), 127_i8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(-128_i8, -128_i8), 0_i8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(127_i8, 127_i8), -2_i8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = false);

        // Unsigned decimal
        assert_eq!(cpu.add(127_u8, 0_u8), 127_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.add(255_u8, 127_u8), 126_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(0_u8, 0_u8), 0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.add(255_u8, 1_u8), 0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(255_u8, 0_u8), 255_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.add(255_u8, 255_u8), 254_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.add(255_u8, 128_u8), 127_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(128_u8, 128_u8), 0_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(127_u8, 127_u8), 254_u8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = false);

        // Hexadecimal
        assert_eq!(cpu.add(0x7F_u8, 0x0_u8), 0x7F_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.add(0xFF_u8, 0x7F_u8), 0x7E_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(0x0_u8, 0x0_u8), 0x0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.add(0xFF_u8, 0x1_u8), 0x0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(0xFF_u8, 0x0_u8), 0xFF_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.add(0xFF_u8, 0xFF_u8), 0xFE_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.add(0xFF_u8, 0x80_u8), 0x7F_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.add(0x80_u8, 0x80_u8), 0x0_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = true, CF = true);

        assert_eq!(cpu.add(0x7F_u8, 0x7F_u8), 0xFE_u8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = false);
    }

    // https://stackoverflow.com/questions/8965923/carry-overflow-subtraction-in-x86#8982549
    //       A                   B                   A - B              Flags
    // ---------------     ----------------    ---------------      -----------------
    // h  |  ud  |   d   | h  |  ud  |   d   | h  |  ud  |   d   || OF | SF | ZF | CF
    // ---+------+-------+----+------+-------+----+------+-------++----+----+----+----
    // FF | 255  |  -1   | FE | 254  |  -2   | 1  |  1   |   1   || 0  | 0  | 0  | 0
    // 7E | 126  |  126  | FF | 255  |  -1   | 7F | 127  |  127  || 0  | 0  | 0  | 1
    // FF | 255  |  -1   | FF | 255  |  -1   | 0  |  0   |   0   || 0  | 0  | 1  | 0
    // FF | 255  |  -1   | 7F | 127  |  127  | 80 | 128  | -128  || 0  | 1  | 0  | 0
    // FE | 254  |  -2   | FF | 255  |  -1   | FF | 255  |  -1   || 0  | 1  | 0  | 1
    // FE | 254  |  -2   | 7F | 127  |  127  | 7F | 127  |  127  || 1  | 0  | 0  | 0
    // 7F | 127  |  127  | FF | 255  |  -1   | 80 | 128  | -128  || 1  | 1  | 0  | 1
    #[test]
    fn sub() {
        let mut cpu = Cpu::default();

        // Decimal
        assert_eq!(cpu.sub(-1_i8, -2_i8), 1_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(126_i8, -1_i8), 127_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.sub(-1_i8, -1_i8), 0_i8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.sub(-1_i8, 127_i8), -128_i8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.sub(-2_i8, -1_i8), -1_i8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.sub(-2_i8, 127_i8), 127_i8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(127_i8, -1_i8), -128_i8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = true);

        // Unsigned decimal
        assert_eq!(cpu.sub(255_u8, 254_u8), 1_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(126_u8, 255_u8), 127_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.sub(255_u8, 255_u8), 0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.sub(255_u8, 127_u8), 128_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.sub(254_u8, 255_u8), 255_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.sub(254_u8, 127_u8), 127_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(127_u8, 255_u8), 128_u8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = true);

        // Hexadecimal
        assert_eq!(cpu.sub(0xFF_u8, 0xFE_u8), 0x1_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(0x7E_u8, 0xFF_u8), 0x7F_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = false, CF = true);

        assert_eq!(cpu.sub(0xFF_u8, 0xFF_u8), 0x0_u8);
        assert_eflags!(cpu, OF = false, SF = false, ZF = true, CF = false);

        assert_eq!(cpu.sub(0xFF_u8, 0x7F_u8), 0x80_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = false);

        assert_eq!(cpu.sub(0xFE_u8, 0xFF_u8), 0xFF_u8);
        assert_eflags!(cpu, OF = false, SF = true, ZF = false, CF = true);

        assert_eq!(cpu.sub(0xFE_u8, 0x7F_u8), 0x7F_u8);
        assert_eflags!(cpu, OF = true, SF = false, ZF = false, CF = false);

        assert_eq!(cpu.sub(0x7F_u8, 0xFF_u8), 0x80_u8);
        assert_eflags!(cpu, OF = true, SF = true, ZF = false, CF = true);
    }
}
