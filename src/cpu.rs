use num::{traits::WrappingAdd, FromPrimitive};

use crate::{instruction::{Instruction, OperandType}, register::Registers, traits::LeastSignificantByte};

pub mod alternate {
    trait Register {}
    trait Register8 {}
    trait Register16 {}
    trait Register32 {}
    trait Rm8 {}
    trait Rm16 {}
    trait Rm32 {}
    trait Read8 {
        fn read(&self) -> u8;
    }
    trait Read16 {
        fn read(&self) -> u16;
    }
    trait Read32 {
        fn read(&self) -> u32;
    }
    trait Write8 {
        fn write(&mut self, value: u8);
    }
    trait Write16 {
        fn write(&mut self, value: u16);
    }
    trait Write32 {
        fn write(&mut self, value: u32);
    }

    struct Eax(u32);
    impl Register for Eax {}
    impl Register32 for Eax {}
    impl Rm32 for Eax {}
    impl Read32 for Eax {
        fn read(&self) -> u32 {
            self.0
        }
    }
    impl Write32 for Eax {
        fn write(&mut self, value: u32) {
            self.0 = value;
        }
    }

    struct Ax(u16);
    impl Register for Ax {}
    impl Register16 for Ax {}
    impl Rm16 for Ax {}
    impl Read16 for Ax {
        fn read(&self) -> u16 {
            self.0
        }
    }
    impl Write16 for Ax {
        fn write(&mut self, value: u16) {
            self.0 = value;
        }
    }

    struct Ah(u8);
    impl Register for Ah {}
    impl Register8 for Ah {}
    impl Rm8 for Ah {}
    impl Read8 for Ah {
        fn read(&self) -> u8 {
            self.0
        }
    }
    impl Write8 for Ah {
        fn write(&mut self, value: u8) {
            self.0 = value;
        }
    }

    struct Al(u8);
    impl Register for Al {}
    impl Register8 for Al {}
    impl Rm8 for Al {}
    impl Read8 for Al {
        fn read(&self) -> u8 {
            self.0
        }
    }
    impl Write8 for Al {
        fn write(&mut self, value: u8) {
            self.0 = value;
        }
    }

    enum RegisterEnum<'a> {
        Eax(&'a Eax),
        Ax(&'a Ax),
        Ah(&'a Ah),
        Al(&'a Al),
    }

    enum Reg {
        Reg8(Reg8),
        Reg16(Reg16),
        Reg32(Reg32),
    }

    enum Reg8 {
        Ah,
        Al,
        Bh,
        Bl,
        Ch,
        Cl,
        Dh,
        Dl,
    }

    enum Reg16 {
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

    enum Reg32 {
        Eax,
        Ebx,
        Ecx,
        Edx,
        Esi,
        Edi,
        Ebp,
        Esp,
    }

    // add eax, 8
    // 1. parse into mnemonic=add, op1=register(eax), op2=immediate(8)
    // 2. resolve matching instruction (ambigious operands/arguments)
    // 3. cast the operands/arguments into the required types
    // 4. call the actual Cpu method with the specific argument types required

    // Rust does not allow self-referential structs, i.e. ax cannot be a subsection of eax. Instead,
    // we must track them as separate fields and update them accordingly on update.
    struct Registers {
        eax: Eax,
        ax: Ax,
        ah: Ah,
        al: Al,
    }

    impl Registers {
        fn foo(&self) {
            let ah = RegisterEnum::Ah(&self.ah);
            let al = RegisterEnum::Al(&self.al);
            let mut cpu = Cpu::new(); 
            cpu.adc_reg8_rm8(ah.0, al.0);
        }
    }

    pub struct Cpu {
        registers: Registers,
    }

    impl Cpu {
        fn new() -> Self {
            Self { registers: Registers { eax: Eax(0), ax: Ax(0), ah: Ah(0), al: Al(0) } }
        }
        pub(crate) fn adc_al_imm8(&mut self, source: u8) {
        }
        pub(crate) fn adc_ax_imm16(&mut self, source: u16) {
        }
        pub(crate) fn adc_eax_imm32(&mut self, source: u32) {
        }
        pub(crate) fn adc_reg8_rm8<D, S>(&mut self, destination: &mut Reg8, source: S)
        where
            S: Rm8 + Read8,
        {
            let result = destination.read() + source.read();
            destination.write(result);
        }
    }

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
}

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
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
        let destination_value = self.registers.get8(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get8(&register.try_into().unwrap()),
        };
        let result = self.add_with_carry(destination_value, source_value);
        self.registers.set8(&destination.try_into().unwrap(), result);
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
        let destination_value = self.registers.get8(&destination.try_into().unwrap());
        let source_value = match &instruction.operands[1].operand_type() {
            OperandType::Immediate(_) => unreachable!(),
            OperandType::Memory(effective_address) => todo!("resolve effective address and get value"),
            OperandType::Register(register) => self.registers.get8(&register.try_into().unwrap()),
        };
        let result = self.add(destination_value, source_value);
        self.registers.set8(&destination.try_into().unwrap(), result);
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
