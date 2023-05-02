use std::ops::{BitAnd, BitOr};

use num_traits::{FromPrimitive, PrimInt, WrappingAdd, WrappingSub};

use crate::{
    instruction::{
        unwrap_operands, EffectiveAddress, Immediate, Operands, RegisterOrMemory16,
        RegisterOrMemory32, RegisterOrMemory8, Size,
    },
    memory::Memory,
    register::{Register16, Register32, Register8, Registers, WithCarry},
    traits::{AsUnsigned, RegisterReadWrite},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
}

#[derive(Clone, Debug, Default)]
pub struct Cpu {
    pub(crate) registers: Registers,
    pub(crate) memory: Memory,
}

impl Cpu {
    /// Performs wrapping addition, adding the carry if required.
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
        result
    }

    /// Performs wrapping subtraction, subtracting the carry if required.
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
        result
    }

    /// Add the two operands and carry together, wrapping if an overflow occurs, and set the
    /// OF, SF, ZF, AF, CF, and PF flags according to the result.
    fn adc<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive + AsUnsigned,
    {
        let result = self.wrapping_add(lhs, rhs, WithCarry::True);
        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Add);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Add);
        self.registers.eflags.compute_parity_flag(result);
        self.registers
            .eflags
            .compute_carry_flag(lhs, rhs, result, Operation::Add);
        result
    }

    pub(crate) fn adc_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.adc(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn adc_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.adc(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn adc_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.adc(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn adc_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.adc(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(&reg8, result);
    }

    pub(crate) fn adc_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.adc(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(&reg16, result);
    }

    pub(crate) fn adc_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.adc(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(&reg32, result);
    }

    pub(crate) fn adc_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.adc(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn adc_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.adc(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn adc_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.adc(rm32.read(self).unwrap(), self.registers.read32(reg32));
        rm32.write(self, result).unwrap();
    }

    /// Add the two operands together, wrapping if an overflow occurs, and set the OF, SF, ZF, AF,
    /// CF, and PF flags according to the result.
    fn add<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingAdd + FromPrimitive + AsUnsigned,
    {
        let result = self.wrapping_add(lhs, rhs, WithCarry::False);
        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Add);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Add);
        self.registers.eflags.compute_parity_flag(result);
        self.registers
            .eflags
            .compute_carry_flag(lhs, rhs, result, Operation::Add);
        result
    }

    pub(crate) fn add_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.add(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn add_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.add(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn add_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.add(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn add_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.add(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(&reg8, result);
    }

    pub(crate) fn add_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.add(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(&reg16, result);
    }

    pub(crate) fn add_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.add(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(&reg32, result);
    }

    pub(crate) fn add_rm8_imm8(&mut self, operands: &Operands) {
        let (rm8, imm8) = unwrap_operands!(operands, RegisterOrMemory8, &Immediate);
        let result = self.add(rm8.read(&self).unwrap(), imm8.0 as u8);
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn add_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.add(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn add_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.add(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn add_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.add(rm32.read(self).unwrap(), self.registers.read32(reg32));
        rm32.write(self, result).unwrap();
    }

    /// Performs a bitwise AND operation. Clears the OF and CF flags, and sets the SF, ZF, and PF
    /// flags depending on the result. The state of the AF flag is undefined.
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

    pub(crate) fn and_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.and(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn and_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.and(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn and_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.and(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn and_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.and(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(reg8, result);
    }

    pub(crate) fn and_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.and(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(reg16, result);
    }

    pub(crate) fn and_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.and(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(reg32, result);
        todo!()
    }

    pub(crate) fn and_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.and(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn and_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.and(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn and_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.and(rm32.read(self).unwrap(), reg32.read(&self.registers));
        rm32.write(self, result).unwrap();
    }

    pub(crate) fn es(&mut self, operands: &Operands) {
        todo!()
    }

    pub(crate) fn daa(&mut self, operands: &Operands) {
        todo!()
    }

    pub(crate) fn lea_reg16_mem(&mut self, operands: &Operands) {
        let (reg16, mem) = unwrap_operands!(operands, &Register16, &EffectiveAddress);
        self.registers.write16(reg16, mem.resolve(self) as u16);
    }

    pub(crate) fn lea_reg32_mem(&mut self, operands: &Operands) {
        let (reg32, mem) = unwrap_operands!(operands, &Register32, &EffectiveAddress);
        self.registers.write32(reg32, mem.resolve(self));
    }

    pub(crate) fn mov_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        rm8.write(self, reg8.read(&self.registers)).unwrap();
    }
    pub(crate) fn mov_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        rm16.write(self, reg16.read(&self.registers)).unwrap();
    }
    pub(crate) fn mov_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        rm32.write(self, reg32.read(&self.registers)).unwrap();
    }
    pub(crate) fn mov_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        self.registers.write8(reg8, rm8.read(self).unwrap());
    }
    pub(crate) fn mov_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        self.registers.write16(reg16, rm16.read(self).unwrap());
    }
    pub(crate) fn mov_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        self.registers.write32(reg32, rm32.read(self).unwrap());
    }

    /// Performs a bitwise inclusive OR operation. The OF and CF flags are cleared, and the SF, ZF,
    /// and PF flags are set according to the result. The AF flag is undefined.
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
    pub(crate) fn or_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.or(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn or_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.or(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn or_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.or(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn or_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.or(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(reg8, result);
    }

    pub(crate) fn or_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.or(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(reg16, result);
    }

    pub(crate) fn or_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.or(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(reg32, result);
    }

    pub(crate) fn or_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.or(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn or_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.or(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn or_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.or(rm32.read(self).unwrap(), self.registers.read32(reg32));
        rm32.write(self, result).unwrap();
    }

    /// Pops a 16-bit (WORD) value off the stack, adjusting the stack pointer as required. Panics
    /// if 16-bit value cannot be read from the location in memory pointed to by ESP.
    fn pop16(&mut self) -> u16 {
        self.registers.shrink_stack(&Size::Word);
        self.memory.read16(self.registers.esp).unwrap()
    }

    /// Pops a 32-bit (DWORD) value off the stack, adjusting the stack pointer as required. Panics
    /// if 32-bit value cannot be read from the location in memory pointed to by ESP.
    fn pop32(&mut self) -> u32 {
        self.registers.shrink_stack(&Size::Dword);
        self.memory.read32(self.registers.esp).unwrap()
    }

    pub(crate) fn pop_ds(&mut self, _operands: &Operands) {
        let popped = self.pop16();
        self.registers.ds = popped;
    }

    pub(crate) fn pop_es(&mut self, _operands: &Operands) {
        let popped = self.pop16();
        self.registers.es = popped;
    }

    pub(crate) fn pop_ss(&mut self, _operands: &Operands) {
        let popped = self.pop16();
        self.registers.ss = popped;
    }

    pub(crate) fn pop_reg16(&mut self, operands: &Operands) {
        let reg16 = unwrap_operands!(operands, &Register16);
        let popped = self.pop16();
        reg16.write(&mut self.registers, popped);
    }

    pub(crate) fn pop_reg32(&mut self, operands: &Operands) {
        let reg32 = unwrap_operands!(operands, &Register32);
        let popped = self.pop32();
        reg32.write(&mut self.registers, popped);
    }

    /// Pushes a 16-bit (WORD) value onto the stack, adjusting the stack pointer as required. Panics
    /// if a 16-bit value cannot be written into memory at the index pointed to by ESP.
    fn push16(&mut self, value: u16) {
        self.registers.grow_stack(&Size::Word);
        self.memory.write16(self.registers.esp, value).unwrap();
    }

    /// Pushes a 32-bit (DWORD) value onto the stack, adjusting the stack pointer as required.
    /// Panics if a 32-bit value cannot be written into memory at the index pointed to by ESP.
    fn push32(&mut self, value: u32) {
        self.registers.grow_stack(&Size::Dword);
        self.memory.write32(self.registers.esp, value).unwrap();
    }

    pub(crate) fn push_cs(&mut self, _operands: &Operands) {
        self.push16(self.registers.cs);
    }

    pub(crate) fn push_ds(&mut self, _operands: &Operands) {
        self.push16(self.registers.ds);
    }

    pub(crate) fn push_es(&mut self, _operands: &Operands) {
        self.push16(self.registers.es);
    }

    pub(crate) fn push_ss(&mut self, _operands: &Operands) {
        self.push16(self.registers.ss);
    }

    pub(crate) fn push_reg16(&mut self, operands: &Operands) {
        let reg16 = unwrap_operands!(operands, &Register16);
        self.push16(reg16.read(&self.registers));
    }

    pub(crate) fn push_reg32(&mut self, operands: &Operands) {
        let reg32 = unwrap_operands!(operands, &Register32);
        self.push32(reg32.read(&self.registers));
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
        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Subtract);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Subtract);
        self.registers.eflags.compute_parity_flag(result);
        self.registers
            .eflags
            .compute_carry_flag(lhs, rhs, result, Operation::Subtract);
        result
    }

    pub(crate) fn sbb_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.sbb(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn sbb_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.sbb(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn sbb_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.sbb(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn sbb_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.sbb(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(reg8, result);
    }

    pub(crate) fn sbb_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.sbb(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(reg16, result);
    }

    pub(crate) fn sbb_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.sbb(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(reg32, result);
    }

    pub(crate) fn sbb_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.sbb(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn sbb_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.sbb(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn sbb_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.sbb(rm32.read(self).unwrap(), self.registers.read32(reg32));
        rm32.write(self, result).unwrap();
    }

    /// Integer subtraction. Adds the source and the carry flag, and subtracts the result from the
    /// destination. Sets the OF, SF, ZF, AF, PF, and CF flags according to the result.
    fn sub<T>(&mut self, lhs: T, rhs: T) -> T
    where
        T: PrimInt + WrappingSub + AsUnsigned + FromPrimitive,
    {
        let result = self.wrapping_sub(lhs, rhs, WithCarry::False);
        self.registers
            .eflags
            .compute_overflow_flag(lhs, rhs, result, Operation::Subtract);
        self.registers.eflags.compute_sign_flag(result);
        self.registers.eflags.compute_zero_flag(result);
        self.registers
            .eflags
            .compute_auxiliary_carry_flag(lhs, rhs, Operation::Subtract);
        self.registers.eflags.compute_parity_flag(result);
        self.registers
            .eflags
            .compute_carry_flag(lhs, rhs, result, Operation::Subtract);
        result
    }

    pub(crate) fn sub_al_imm8(&mut self, operands: &Operands) {
        let (_al, imm8) = unwrap_operands!(operands, &Register8, &Immediate);
        let result = self.sub(self.registers.get_al(), imm8.0 as u8);
        self.registers.set_al(result);
    }

    pub(crate) fn sub_ax_imm16(&mut self, operands: &Operands) {
        let (_ax, imm16) = unwrap_operands!(operands, &Register16, &Immediate);
        let result = self.sub(self.registers.get_ax(), imm16.0 as u16);
        self.registers.set_ax(result);
    }

    pub(crate) fn sub_eax_imm32(&mut self, operands: &Operands) {
        let (_eax, imm32) = unwrap_operands!(operands, &Register32, &Immediate);
        let result = self.sub(self.registers.get_eax(), imm32.0 as u32);
        self.registers.set_eax(result);
    }

    pub(crate) fn sub_reg8_rm8(&mut self, operands: &Operands) {
        let (reg8, rm8) = unwrap_operands!(operands, &Register8, RegisterOrMemory8);
        let result = self.sub(reg8.read(&self.registers), rm8.read(self).unwrap());
        self.registers.write8(reg8, result);
    }

    pub(crate) fn sub_reg16_rm16(&mut self, operands: &Operands) {
        let (reg16, rm16) = unwrap_operands!(operands, &Register16, RegisterOrMemory16);
        let result = self.sub(reg16.read(&self.registers), rm16.read(self).unwrap());
        self.registers.write16(reg16, result);
    }

    pub(crate) fn sub_reg32_rm32(&mut self, operands: &Operands) {
        let (reg32, rm32) = unwrap_operands!(operands, &Register32, RegisterOrMemory32);
        let result = self.sub(self.registers.read32(reg32), rm32.read(self).unwrap());
        self.registers.write32(reg32, result);
    }

    pub(crate) fn sub_rm8_reg8(&mut self, operands: &Operands) {
        let (rm8, reg8) = unwrap_operands!(operands, RegisterOrMemory8, &Register8);
        let result = self.sub(rm8.read(self).unwrap(), reg8.read(&self.registers));
        rm8.write(self, result).unwrap();
    }

    pub(crate) fn sub_rm16_reg16(&mut self, operands: &Operands) {
        let (rm16, reg16) = unwrap_operands!(operands, RegisterOrMemory16, &Register16);
        let result = self.sub(rm16.read(self).unwrap(), reg16.read(&self.registers));
        rm16.write(self, result).unwrap();
    }

    pub(crate) fn sub_rm32_reg32(&mut self, operands: &Operands) {
        let (rm32, reg32) = unwrap_operands!(operands, RegisterOrMemory32, &Register32);
        let result = self.sub(rm32.read(self).unwrap(), reg32.read(&self.registers));
        rm32.write(self, result).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{NasmStr, Operand};

    macro_rules! assert_eflags {
        (@ $cpu:ident, CF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_carry_flag(), $expected, "CF is incorrect")
        };
        (@ $cpu:ident, PF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_parity_flag(), $expected, "PF is incorrect")
        };
        (@ $cpu:ident, AF=$expected:literal) => {
            assert_eq!(
                $cpu.registers.eflags.get_auxiliary_carry_flag(),
                $expected,
                "AF is incorrect"
            )
        };
        (@ $cpu:ident, ZF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_zero_flag(), $expected, "ZF is incorrect")
        };
        (@ $cpu:ident, SF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_sign_flag(), $expected, "SF is incorrect")
        };
        (@ $cpu:ident, OF=$expected:literal) => {
            assert_eq!($cpu.registers.eflags.get_overflow_flag(), $expected, "OF is incorrect")
        };
        ($cpu:ident, $($flag:ident=$expected:literal),+) => {
            $(assert_eflags!(@ $cpu, $flag=$expected));+
        };
    }

    macro_rules! operands {
        () => { Operands(vec![]) };
        ($operand:literal) => { Operands(vec![Operand::try_from(&NasmStr($operand)).unwrap()])};
        ($operand_a:literal, $operand_b:literal) => {
            {
                let mut operands = operands!($operand_a);
                operands.0.append(&mut operands!($operand_b).0);
                operands
            }
        };
        ($operand:literal, $($tail:tt)*) => {
            {
                operands!($operand).0.append(&mut operands!($($tail)*).0)
            }
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
    // TODO: Test for AF and PF.
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
    // TODO: Why can't you have the other flag combinations e.g. OF + ZF?
    // TODO: Test for other 2 flags which are set.
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

    #[test]
    fn and() {
        let mut cpu = Cpu::default();

        cpu.registers.eflags.set_overflow_flag(true);
        cpu.registers.eflags.set_carry_flag(true);

        assert_eq!(
            cpu.and(0b0000_0001_u8, 0b1111_1111_u8),
            0b0000_0001_u8 & 0b1111_1111_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = false,
            PF = false
        );

        assert_eq!(
            cpu.and(0b0000_0011_u8, 0b1111_1111_u8),
            0b0000_0011_u8 & 0b1111_1111_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = false,
            PF = true
        );

        assert_eq!(
            cpu.and(0b0000_0000_u8, 0b1111_1111_u8),
            0b0000_0000_u8 & 0b1111_1111_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = true,
            PF = true
        );

        assert_eq!(
            cpu.and(0b1000_0000_u8, 0b1111_1111_u8),
            0b1000_0000_u8 & 0b1111_1111_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = true,
            ZF = false,
            PF = false
        );
    }

    #[test]
    fn lea_reg16_mem() {
        let mut cpu = Cpu::default();
        cpu.registers.set_ebx(10);
        cpu.lea_reg16_mem(&operands!("ax", "[ebx]"));
        assert_eq!(cpu.registers.get_ax(), 10);
    }

    #[test]
    fn lea_reg32_mem() {
        let mut cpu = Cpu::default();
        cpu.registers.set_ebx(10);
        cpu.lea_reg32_mem(&operands!("eax", "[ebx]"));
        assert_eq!(cpu.registers.get_eax(), 10);
    }

    #[test]
    fn mov_rm8_reg8() {
        let mut cpu = Cpu::default();

        cpu.registers.set_bh(1);
        cpu.mov_rm8_reg8(&operands!("ah", "bh"));
        assert_eq!(cpu.registers.get_ah(), 1);

        cpu.mov_rm8_reg8(&operands!("BYTE [0]", "bh"));
        assert_eq!(cpu.memory.read8(0).unwrap(), 1);
    }

    #[test]
    fn mov_rm16_reg16() {
        let mut cpu = Cpu::default();

        cpu.registers.set_bx(1);
        cpu.mov_rm16_reg16(&operands!("ax", "bx"));
        assert_eq!(cpu.registers.get_ax(), 1);

        cpu.mov_rm16_reg16(&operands!("WORD [0]", "bx"));
        assert_eq!(cpu.memory.read16(0).unwrap(), 1);
    }

    #[test]
    fn mov_rm32_reg32() {
        let mut cpu = Cpu::default();

        cpu.registers.set_ebx(1);
        cpu.mov_rm32_reg32(&operands!("eax", "ebx"));
        assert_eq!(cpu.registers.get_eax(), 1);

        cpu.mov_rm32_reg32(&operands!("BYTE [0]", "ebx"));
        assert_eq!(cpu.memory.read32(0).unwrap(), 1);
    }

    #[test]
    fn mov_reg8_rm8() {
        let mut cpu = Cpu::default();

        cpu.registers.set_al(1);
        cpu.registers.set_bl(2);

        cpu.mov_reg8_rm8(&operands!("al", "[0]"));
        assert_eq!(cpu.registers.get_al(), 0);

        cpu.mov_reg8_rm8(&operands!("al", "bl"));
        assert_eq!(cpu.registers.get_al(), 2);
    }

    #[test]
    fn mov_reg16_rm16() {
        let mut cpu = Cpu::default();

        cpu.registers.set_ax(1);
        cpu.registers.set_bx(2);

        cpu.mov_reg16_rm16(&operands!("ax", "[0]"));
        assert_eq!(cpu.registers.get_ax(), 0);

        cpu.mov_reg16_rm16(&operands!("ax", "bx"));
        assert_eq!(cpu.registers.get_ax(), 2);
    }

    #[test]
    fn mov_reg32_rm32() {
        let mut cpu = Cpu::default();

        cpu.registers.set_eax(1);
        cpu.registers.set_ebx(2);

        cpu.mov_reg32_rm32(&operands!("eax", "[0]"));
        assert_eq!(cpu.registers.get_eax(), 0);

        cpu.mov_reg32_rm32(&operands!("eax", "ebx"));
        assert_eq!(cpu.registers.get_eax(), 2);
    }

    #[test]
    fn or() {
        let mut cpu = Cpu::default();

        cpu.registers.eflags.set_overflow_flag(true);
        cpu.registers.eflags.set_carry_flag(true);

        assert_eq!(
            cpu.or(0b0000_0001_u8, 0b0000_0000_u8),
            0b0000_0001_u8 | 0b0000_0000_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = false,
            PF = false
        );

        assert_eq!(
            cpu.or(0b0000_0011_u8, 0b0000_0000_u8),
            0b0000_0011_u8 | 0b0000_0000_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = false,
            PF = true
        );

        assert_eq!(
            cpu.or(0b0000_0000_u8, 0b0000_0000_u8),
            0b0000_0000_u8 | 0b0000_0000_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = false,
            ZF = true,
            PF = true
        );

        assert_eq!(
            cpu.or(0b1000_0000_u8, 0b0000_0000_u8),
            0b1000_0000_u8 | 0b0000_0000_u8
        );
        assert_eflags!(
            cpu,
            OF = false,
            CF = false,
            SF = true,
            ZF = false,
            PF = false
        );
    }

    #[test]
    fn pop() {
        let mut cpu = Cpu::default();
        cpu.registers.esp = 128;

        cpu.memory.write16(130, u16::MAX).unwrap();
        assert_eq!(cpu.pop16(), u16::MAX);
        assert_eq!(cpu.registers.esp, 130);

        cpu.memory.write32(134, u32::MAX).unwrap();
        assert_eq!(cpu.pop32(), u32::MAX);
        assert_eq!(cpu.registers.esp, 134);
    }

    #[test]
    fn push() {
        let mut cpu = Cpu::default();
        cpu.registers.esp = 128;

        cpu.push16(u16::MAX);
        assert_eq!(cpu.registers.esp, 126);
        assert_eq!(cpu.memory.read16(126).unwrap(), u16::MAX);

        cpu.push32(u32::MAX);
        assert_eq!(cpu.registers.esp, 122);
        assert_eq!(cpu.memory.read32(122).unwrap(), u32::MAX);
    }
}
