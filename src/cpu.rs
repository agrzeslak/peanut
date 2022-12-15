use num::{traits::WrappingAdd, FromPrimitive};

use crate::{instruction::{Instruction, OperandType}, register::Registers, traits::LeastSignificantByte};

#[derive(Clone, Debug, Default)]
pub struct Cpu {
    pub(crate) registers: Registers,
}

impl Cpu {
    /// Add the two operands and carry together, wrapping if an overflow occurs, and set the
    /// appropriate flags.
    // TODO: Tests, especially for wrapping.
    fn add_with_carry<T>(&mut self, a: T, b: T) -> T
    where
        T: LeastSignificantByte + WrappingAdd + FromPrimitive
    {
        let carry = self.registers.eflags.get_carry_flag() as u8;
        let result = a + b + FromPrimitive::from_u8(carry).unwrap();
        self.registers.eflags.compute_parity_flag(&result) ;
        result
    }
    pub(crate) fn adc_al_imm8(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add_with_carry(self.registers.get_al(), immediate.parsed() as u8);
        self.registers.set_al(result);
    }
    pub(crate) fn adc_ax_imm16(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add_with_carry(self.registers.get_ax(), immediate.parsed() as u16);
        self.registers.set_ax(result);
    }
    pub(crate) fn adc_eax_imm32(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add_with_carry(self.registers.get_eax(), immediate.parsed() as u32);
        self.registers.set_eax(result);
    }
    pub(crate) fn adc_reg8_rm8(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.read8(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.read8(&register.try_into().unwrap()),
        };
        let result = self.add_with_carry(destination_value, source_value);
        self.registers.write8(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn adc_reg16_rm16(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.get16(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get16(&register.try_into().unwrap()),
        };
        let result = self.add_with_carry(destination_value, source_value);
        self.registers.set16(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn adc_reg32_rm32(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.get32(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get32(&register.try_into().unwrap()),
        };
        let result = self.add_with_carry(destination_value, source_value);
        self.registers.set32(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn adc_rm8_reg8(&mut self, instruction: &Instruction) {
        // let destination = match &instruction.operands[0].operand_type() {
        //     OperandType::Immediate(_) => unreachable!(),
        //     OperandType::Memory(effective_address) => todo!("resolve effective address to write to"),
        //     OperandType::Register(register) => self.registers.get8(&registers.try_into().unwrap()),
        // };
        // let result = self.add_with_carry(destination_value, source_value);
        todo!();
    }
    pub(crate) fn adc_rm16_reg16(&mut self, instruction: &Instruction) {  }
    pub(crate) fn adc_rm32_reg32(&mut self, instruction: &Instruction) {  }
    /// Add the two operands together, wrapping if an overflow occurs, and set the appropriate
    /// flags.
    // TODO: Tests, especially for wrapping.
    fn add<T>(&mut self, a: T, b: T) -> T
    where
        T: LeastSignificantByte + WrappingAdd
    {
        let result = a + b;
        self.registers.eflags.compute_parity_flag(&result);
        result
    }
    pub(crate) fn add_al_imm8(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add(self.registers.get_al(), immediate.parsed() as u8);
        self.registers.set_al(result);
    }
    pub(crate) fn add_ax_imm16(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add(self.registers.get_ax(), immediate.parsed() as u16);
        self.registers.set_ax(result);
    }
    pub(crate) fn add_eax_imm32(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.add(self.registers.get_eax(), immediate.parsed() as u32);
        self.registers.set_eax(result);
    }
    pub(crate) fn add_reg8_rm8(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.read8(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.read8(&register.try_into().unwrap()),
        };
        let result = self.add(destination_value, source_value);
        self.registers.write8(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn add_reg16_rm16(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.get16(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get16(&register.try_into().unwrap()),
        };
        let result = self.add(destination_value, source_value);
        self.registers.set16(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn add_reg32_rm32(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.get32(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get32(&register.try_into().unwrap()),
        };
        let result = self.add(destination_value, source_value);
        self.registers.set32(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn add_rm8_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn add_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn add_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn add_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_al_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_ax_imm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_eax_imm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_reg8_rm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn and_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn es(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn daa(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_al_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_ax_imm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_eax_imm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_reg8_rm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn or_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn pop_ds(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn pop_es(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn pop_ss(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn push_cs(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn push_ds(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn push_es(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn push_ss(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_al_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_ax_imm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_eax_imm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_reg8_rm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sbb_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_al_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_ax_imm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_eax_imm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_reg8_rm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn sub_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
}
