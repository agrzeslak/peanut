use std::ops::BitAnd;

use num_traits::{FromPrimitive, PrimInt, WrappingAdd};

use crate::{
    instruction::{
        unwrap_operands, Immediate, Instruction, RegisterOrMemory16, RegisterOrMemory32,
        RegisterOrMemory8,
    },
    register::{Register16, Register32, Register8, Registers},
};

#[derive(Clone, Debug, Default)]
pub struct Cpu {
    pub(crate) registers: Registers,
}

impl Cpu {
    /// Add the two operands and carry together, wrapping if an overflow occurs, and set the
    /// OF, SF, ZF, AF, CF, and PF flags according to the result.
    // TODO: Tests, especially for wrapping.
    // TODO: Document flags which are set.
    fn adc<T>(&mut self, a: T, b: T) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive,
    {
        let carry = self.registers.eflags.get_carry_flag() as u8;
        let result = a + b + FromPrimitive::from_u8(carry).unwrap();
        self.registers.eflags.compute_parity_flag(result);
        // TODO: OF, SF, ZF, AF, CF, and PF
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
    // TODO: Tests, especially for wrapping.
    fn add<T>(&mut self, a: T, b: T) -> T
    where
        T: PrimInt + WrappingAdd,
    {
        let result = a + b;
        self.registers.eflags.compute_parity_flag(result);
        // TODO: OF, SF, ZF, AF, CF, and PF
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
    fn and<T>(&mut self, a: T, b: T) -> T
    where
        T: PrimInt + BitAnd<Output = T>,
    {
        let result = a & b;
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

    pub(crate) fn or_al_imm8(&mut self, instruction: &Instruction) {
        let (_al, imm8) = unwrap_operands!(instruction, &Register8, &Immediate);
        todo!()
    }

    pub(crate) fn or_ax_imm16(&mut self, instruction: &Instruction) {
        let (_ax, imm16) = unwrap_operands!(instruction, &Register16, &Immediate);
        todo!()
    }

    pub(crate) fn or_eax_imm32(&mut self, instruction: &Instruction) {
        let (_eax, imm32) = unwrap_operands!(instruction, &Register32, &Immediate);
        todo!()
    }

    pub(crate) fn or_reg8_rm8(&mut self, instruction: &Instruction) {
        let (reg8, rm8) = unwrap_operands!(instruction, &Register8, RegisterOrMemory8);
        todo!()
    }

    pub(crate) fn or_reg16_rm16(&mut self, instruction: &Instruction) {
        let (reg16, rm16) = unwrap_operands!(instruction, &Register16, RegisterOrMemory16);
        todo!()
    }

    pub(crate) fn or_reg32_rm32(&mut self, instruction: &Instruction) {
        let (reg32, rm32) = unwrap_operands!(instruction, &Register32, RegisterOrMemory32);
        todo!()
    }

    pub(crate) fn or_rm8_reg8(&mut self, instruction: &Instruction) {
        let (rm8, reg8) = unwrap_operands!(instruction, RegisterOrMemory8, &Register8);
        todo!()
    }

    pub(crate) fn or_rm16_reg16(&mut self, instruction: &Instruction) {
        let (rm16, reg16) = unwrap_operands!(instruction, RegisterOrMemory16, &Register16);
        todo!()
    }

    pub(crate) fn or_rm32_reg32(&mut self, instruction: &Instruction) {
        let (rm32, reg32) = unwrap_operands!(instruction, RegisterOrMemory32, &Register32);
        todo!()
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
