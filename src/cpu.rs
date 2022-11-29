use crate::{instruction::{Instruction, OperandType}, register::Registers};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub(crate) fn adc_al_imm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_ax_imm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_eax_imm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_reg8_rm8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_rm8_reg8(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_rm16_reg16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn adc_rm32_reg32(&mut self, instruction: &Instruction) { todo!() }
    // o..szapc = overflow, sign, zero, auxiliary, parity, carry
    // FIXME: The parity flag (amongst other flags) are computed in each instruction invocation.
    //        This is repetitive and prone to an error by ommission being made. Instead, we should
    //        centralise this somewhere where it may be abstracted away. E.g. all additions go
    //        through a single interface where the relevant flags are computed there. Unsure if
    //        this is feasible or practical, but it's worth considering once this code shakes out a
    //        bit further and I understand the x86 architecture better.
    pub(crate) fn add_al_imm8(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.registers.get_al() + immediate.parsed() as u8;
        self.registers.eflags.compute_parity_flag(result);
        self.registers.set_al(result);
    }
    pub(crate) fn add_ax_imm16(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.registers.get_ax() + immediate.parsed() as u16;
        self.registers.eflags.compute_parity_flag(result.to_le_bytes()[0]);
        self.registers.set_ax(result);
    }
    pub(crate) fn add_eax_imm32(&mut self, instruction: &Instruction) {
        let immediate = instruction.unwrap_immediate_operand(0);
        let result = self.registers.get_eax() + immediate.parsed() as u32;
        self.registers.eflags.compute_parity_flag(result.to_le_bytes()[0]);
        self.registers.set_eax(result);
    }
    pub(crate) fn add_reg8_rm8(&mut self, instruction: &Instruction) {
        let destination = instruction.unwrap_register_operand(0);
        let destination_value = self.registers.get8(&destination.try_into().unwrap());
        let value = match &instruction.operands.get(1).unwrap().operand_type() {
            OperandType::Immediate(_) => panic!("immediate value cannot be used for rm operand"),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get8(&register.try_into().unwrap()),
        };
        let result = destination_value + value;
        self.registers.eflags.compute_parity_flag(result);
        self.registers.set8(&destination.try_into().unwrap(), result);
    }
    pub(crate) fn add_reg16_rm16(&mut self, instruction: &Instruction) { todo!() }
    pub(crate) fn add_reg32_rm32(&mut self, instruction: &Instruction) { todo!() }
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
