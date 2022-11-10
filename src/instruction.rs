use crate::{cpu::Cpu, error::Error, parser::NasmInstructionStrParser, register::Register};

#[derive(Debug)]
enum InstructionOperandFormat {
    Cs,
    Ds,
    Es,
    Ss,
    Fs,
    Gs,
    Const3,
    Imm8,
    Imm16,
    Imm32,
    Reg16,
    Reg32,
    Reg8Imm8,
    Reg16Imm16,
    Reg32Imm32,
    Rel8,
    Rel16,
    Rel32,
    Rm8,
    Rm16,
    Rm32,
    Reg8Rm8,
    Reg16Rm16,
    Reg32Rm32,
    Rm8Reg8,
    Rm16Reg16,
    Rm32Reg32,
    Rm16Sreg,
    Rm32Sreg,
    Rm8Imm8,
    Rm16Imm16,
    Rm16Imm8,
    Rm32Imm8,
    Rm32Imm32,
    Reg16Rm16Imm8,
    Reg16Rm16Imm16,
    Reg32Rm32Imm8,
    Reg32Rm32Imm32,
    Reg16Mem16,
    Reg32Mem32,
    SregRm16,
    SregRm32,
    Rm8Const1,
    Rm16Const1,
    Rm32Const1,
    Far16,
    Far32,
    Rm8Cl,
    Rm16Cl,
    Rm32Cl,
    Reg32Cr,
    Reg32Dr,
    CrReg32,
    DrReg32,
    Reg16Rm8,
    Reg32Rm8,
    Reg32Rm16,
    Rm16Reg16Imm8,
    Rm32Reg32Imm8,
    Rm16Reg16Cl,
    Rm32Reg32Cl,
    AlImm8,
    AxImm16,
    EaxImm32,
    Imm16Imm16,
    Imm16Imm32,
    AxReg16,
    EaxReg32,
    AxImm8,
    EaxImm8,
    AlMoffs8,
    AxMoffs16,
    EaxMoffs32,
    Moffs8Al,
    Moffs16Ax,
    Moffs32Eax,
    AlDx,
    AxDx,
    EaxDx,
    DxAl,
    DxAx,
    DxEax,
    Imm8Al,
    Imm8Ax,
    Imm8Eax,
    Imm8Imm16,
    Reg8Cl,
    None,
}

type CpuFunction = fn(&mut Cpu, &Instruction);

struct OperandFunctionMap {
    pub instruction_operand_format: InstructionOperandFormat,
    pub cpu_function: CpuFunction,
}

impl From<(InstructionOperandFormat, CpuFunction)> for OperandFunctionMap {
    fn from(format_and_function: (InstructionOperandFormat, CpuFunction)) -> Self {
        Self {
            instruction_operand_format: format_and_function.0,
            cpu_function: format_and_function.1,
        }
    }
}

/// A valid instruction's signature, which may be matched against to determine what x86 instruction
/// should be performed.
pub(crate) struct InstructionDescriptor<'a> {
    opcode: u32,
    mnemonic: &'a str,
    operand_function_map_8: Option<OperandFunctionMap>,
    operand_function_map_16: Option<OperandFunctionMap>,
    operand_function_map_32: Option<OperandFunctionMap>,
    lock_prefix: bool,
}

macro_rules! expand_operand_function_mapping {
    ($instruction_operand_format:ident, $cpu_function:ident) => {
        Some(OperandFunctionMap {
            instruction_operand_format: InstructionOperandFormat::$instruction_operand_format,
            cpu_function: Cpu::$cpu_function,
        })
    };
    () => {
        None::<OperandFunctionMap>
    };
}

macro_rules! build {
    (
        $opcode:literal,
        $mnemonic:literal,
        ($($mapping_8:tt)*),
        ($($mapping_16:tt)*),
        ($($mapping_32:tt)*),
        $lock_prefix:literal
    ) => {
        InstructionDescriptor {
            opcode: $opcode,
            mnemonic: $mnemonic,
            operand_function_map_8: expand_operand_function_mapping!($($mapping_8)*),
            operand_function_map_16: expand_operand_function_mapping!($($mapping_16)*),
            operand_function_map_32: expand_operand_function_mapping!($($mapping_32)*),
            lock_prefix: $lock_prefix,
        }
    }
}

// TODO: Hash maps for op code and mnemonic look-ups.
const INSTRUCTION_DESCRIPTORS: [InstructionDescriptor; 254] = [
    build!(0x00, "ADD", (Rm8Reg8, add_rm8_reg8), (), (), true),
    build!(
        0x01,
        "ADD",
        (),
        (Rm16Reg16, add_rm16_reg16),
        (Rm32Reg32, add_rm32_reg32),
        true
    ),
    build!(0x02, "ADD", (Reg8Rm8, add_reg8_rm8), (), (), false),
    build!(
        0x03,
        "ADD",
        (),
        (Reg16Rm16, add_reg16_rm16),
        (Reg32Rm32, add_reg32_rm32),
        false
    ),
    build!(0x04, "ADD", (AlImm8, add_al_imm8), (), (), false),
    build!(
        0x05,
        "ADD",
        (),
        (AxImm16, add_ax_imm16),
        (EaxImm32, add_eax_imm32),
        false
    ),
    build!(0x06, "PUSH", (), (Es, push_es), (), false),
    build!(0x07, "POP", (), (Es, pop_es), (), false),
    build!(0x08, "OR", (Rm8Reg8, or_rm8_reg8), (), (), true),
    build!(
        0x09,
        "OR",
        (),
        (Rm16Reg16, or_rm16_reg16),
        (Rm32Reg32, or_rm32_reg32),
        true
    ),
    build!(0x0a, "OR", (Reg8Rm8, or_reg8_rm8), (), (), false),
    build!(
        0x0b,
        "OR",
        (),
        (Reg16Rm16, or_reg16_rm16),
        (Reg32Rm32, or_reg32_rm32),
        false
    ),
    build!(0x0c, "OR", (AlImm8, or_al_imm8), (), (), false),
    build!(
        0x0d,
        "OR",
        (),
        (AxImm16, or_ax_imm16),
        (EaxImm32, or_eax_imm32),
        false
    ),
    build!(0x0e, "PUSH", (), (Cs, push_cs), (), false),
    build!(0x10, "ADC", (Rm8Reg8, adc_rm8_reg8), (), (), true),
    build!(
        0x11,
        "ADC",
        (),
        (Rm16Reg16, adc_rm16_reg16),
        (Rm32Reg32, adc_rm32_reg32),
        true
    ),
    build!(0x12, "ADC", (Reg8Rm8, adc_reg8_rm8), (), (), false),
    build!(
        0x13,
        "ADC",
        (),
        (Reg16Rm16, adc_reg16_rm16),
        (Reg32Rm32, adc_reg32_rm32),
        false
    ),
    build!(0x14, "ADC", (AlImm8, adc_al_imm8), (), (), false),
    build!(
        0x15,
        "ADC",
        (),
        (AxImm16, adc_ax_imm16),
        (EaxImm32, adc_eax_imm32),
        false
    ),
    build!(0x16, "PUSH", (), (Ss, push_ss), (), false),
    build!(0x17, "POP", (), (Ss, pop_ss), (), false),
    build!(0x18, "SBB", (Rm8Reg8, sbb_rm8_reg8), (), (), true),
    build!(
        0x19,
        "SBB",
        (),
        (Rm16Reg16, sbb_rm16_reg16),
        (Rm32Reg32, sbb_rm32_reg32),
        true
    ),
    build!(0x1a, "SBB", (Reg8Rm8, sbb_reg8_rm8), (), (), false),
    build!(
        0x1b,
        "SBB",
        (),
        (Reg16Rm16, sbb_reg16_rm16),
        (Reg32Rm32, sbb_reg32_rm32),
        false
    ),
    build!(0x1c, "SBB", (AlImm8, sbb_al_imm8), (), (), false),
    build!(
        0x1d,
        "SBB",
        (),
        (AxImm16, sbb_ax_imm16),
        (EaxImm32, sbb_eax_imm32),
        false
    ),
    build!(0x1e, "PUSH", (), (Ds, push_ds), (), false),
    build!(0x1f, "POP", (), (Ds, pop_ds), (), false),
    build!(0x20, "AND", (Rm8Reg8, and_rm8_reg8), (), (), true),
    build!(
        0x21,
        "AND",
        (),
        (Rm16Reg16, and_rm16_reg16),
        (Rm32Reg32, and_rm32_reg32),
        true
    ),
    build!(0x22, "AND", (Reg8Rm8, and_reg8_rm8), (), (), false),
    build!(
        0x23,
        "AND",
        (),
        (Reg16Rm16, and_reg16_rm16),
        (Reg32Rm32, and_reg32_rm32),
        false
    ),
    build!(0x24, "AND", (AlImm8, and_al_imm8), (), (), false),
    build!(
        0x25,
        "AND",
        (),
        (AxImm16, and_ax_imm16),
        (EaxImm32, and_eax_imm32),
        false
    ),
    build!(0x26, "ES", (), (None, es), (), false),
    build!(0x27, "DAA", (None, daa), (), (), false),
    build!(0x28, "SUB", (Rm8Reg8, sub_rm8_reg8), (), (), true),
    build!(
        0x29,
        "SUB",
        (),
        (Rm16Reg16, sub_rm16_reg16),
        (Rm32Reg32, sub_rm32_reg32),
        true
    ),
    build!(0x2a, "SUB", (Reg8Rm8, sub_reg8_rm8), (), (), false),
    build!(
        0x2b,
        "SUB",
        (),
        (Reg16Rm16, sub_reg16_rm16),
        (Reg32Rm32, sub_reg32_rm32),
        false
    ),
    build!(0x2c, "SUB", (AlImm8, sub_al_imm8), (), (), false),
    build!(
        0x2d,
        "SUB",
        (),
        (AxImm16, sub_ax_imm16),
        (EaxImm32, sub_eax_imm32),
        false
    ),
    build!(0x2e, "", (), (), (), false),
    build!(0x2f, "", (), (), (), false),
    build!(0x30, "", (), (), (), false),
    build!(0x31, "", (), (), (), false),
    build!(0x32, "", (), (), (), false),
    build!(0x33, "", (), (), (), false),
    build!(0x34, "", (), (), (), false),
    build!(0x35, "", (), (), (), false),
    build!(0x36, "", (), (), (), false),
    build!(0x37, "", (), (), (), false),
    build!(0x38, "", (), (), (), false),
    build!(0x39, "", (), (), (), false),
    build!(0x3a, "", (), (), (), false),
    build!(0x3b, "", (), (), (), false),
    build!(0x3c, "", (), (), (), false),
    build!(0x3d, "", (), (), (), false),
    build!(0x3e, "", (), (), (), false),
    build!(0x3f, "", (), (), (), false),
    build!(0x40, "", (), (), (), false),
    build!(0x41, "", (), (), (), false),
    build!(0x42, "", (), (), (), false),
    build!(0x43, "", (), (), (), false),
    build!(0x44, "", (), (), (), false),
    build!(0x45, "", (), (), (), false),
    build!(0x46, "", (), (), (), false),
    build!(0x47, "", (), (), (), false),
    build!(0x48, "", (), (), (), false),
    build!(0x49, "", (), (), (), false),
    build!(0x4a, "", (), (), (), false),
    build!(0x4b, "", (), (), (), false),
    build!(0x4c, "", (), (), (), false),
    build!(0x4d, "", (), (), (), false),
    build!(0x4e, "", (), (), (), false),
    build!(0x4f, "", (), (), (), false),
    build!(0x50, "", (), (), (), false),
    build!(0x51, "", (), (), (), false),
    build!(0x52, "", (), (), (), false),
    build!(0x53, "", (), (), (), false),
    build!(0x54, "", (), (), (), false),
    build!(0x55, "", (), (), (), false),
    build!(0x56, "", (), (), (), false),
    build!(0x57, "", (), (), (), false),
    build!(0x58, "", (), (), (), false),
    build!(0x59, "", (), (), (), false),
    build!(0x5a, "", (), (), (), false),
    build!(0x5b, "", (), (), (), false),
    build!(0x5c, "", (), (), (), false),
    build!(0x5d, "", (), (), (), false),
    build!(0x5e, "", (), (), (), false),
    build!(0x5f, "", (), (), (), false),
    build!(0x60, "", (), (), (), false),
    build!(0x61, "", (), (), (), false),
    build!(0x62, "", (), (), (), false),
    build!(0x63, "", (), (), (), false),
    build!(0x64, "", (), (), (), false),
    build!(0x65, "", (), (), (), false),
    build!(0x66, "", (), (), (), false),
    build!(0x67, "", (), (), (), false),
    build!(0x68, "", (), (), (), false),
    build!(0x69, "", (), (), (), false),
    build!(0x6a, "", (), (), (), false),
    build!(0x6b, "", (), (), (), false),
    build!(0x6c, "", (), (), (), false),
    build!(0x6d, "", (), (), (), false),
    build!(0x6e, "", (), (), (), false),
    build!(0x6f, "", (), (), (), false),
    build!(0x70, "", (), (), (), false),
    build!(0x71, "", (), (), (), false),
    build!(0x72, "", (), (), (), false),
    build!(0x73, "", (), (), (), false),
    build!(0x74, "", (), (), (), false),
    build!(0x75, "", (), (), (), false),
    build!(0x76, "", (), (), (), false),
    build!(0x77, "", (), (), (), false),
    build!(0x78, "", (), (), (), false),
    build!(0x79, "", (), (), (), false),
    build!(0x7a, "", (), (), (), false),
    build!(0x7b, "", (), (), (), false),
    build!(0x7c, "", (), (), (), false),
    build!(0x7d, "", (), (), (), false),
    build!(0x7e, "", (), (), (), false),
    build!(0x7f, "", (), (), (), false),
    build!(0x80, "", (), (), (), false),
    build!(0x81, "", (), (), (), false),
    build!(0x82, "", (), (), (), false),
    build!(0x83, "", (), (), (), false),
    build!(0x84, "", (), (), (), false),
    build!(0x85, "", (), (), (), false),
    build!(0x86, "", (), (), (), false),
    build!(0x87, "", (), (), (), false),
    build!(0x88, "", (), (), (), false),
    build!(0x89, "", (), (), (), false),
    build!(0x8a, "", (), (), (), false),
    build!(0x8b, "", (), (), (), false),
    build!(0x8c, "", (), (), (), false),
    build!(0x8d, "", (), (), (), false),
    build!(0x8e, "", (), (), (), false),
    build!(0x8f, "", (), (), (), false),
    build!(0x90, "", (), (), (), false),
    build!(0x91, "", (), (), (), false),
    build!(0x92, "", (), (), (), false),
    build!(0x93, "", (), (), (), false),
    build!(0x94, "", (), (), (), false),
    build!(0x95, "", (), (), (), false),
    build!(0x96, "", (), (), (), false),
    build!(0x97, "", (), (), (), false),
    build!(0x98, "", (), (), (), false),
    build!(0x99, "", (), (), (), false),
    build!(0x9a, "", (), (), (), false),
    build!(0x9b, "", (), (), (), false),
    build!(0x9c, "", (), (), (), false),
    build!(0x9d, "", (), (), (), false),
    build!(0x9e, "", (), (), (), false),
    build!(0x9f, "", (), (), (), false),
    build!(0xa0, "", (), (), (), false),
    build!(0xa1, "", (), (), (), false),
    build!(0xa2, "", (), (), (), false),
    build!(0xa3, "", (), (), (), false),
    build!(0xa4, "", (), (), (), false),
    build!(0xa5, "", (), (), (), false),
    build!(0xa6, "", (), (), (), false),
    build!(0xa7, "", (), (), (), false),
    build!(0xa8, "", (), (), (), false),
    build!(0xa9, "", (), (), (), false),
    build!(0xaa, "", (), (), (), false),
    build!(0xab, "", (), (), (), false),
    build!(0xac, "", (), (), (), false),
    build!(0xad, "", (), (), (), false),
    build!(0xae, "", (), (), (), false),
    build!(0xaf, "", (), (), (), false),
    build!(0xb0, "", (), (), (), false),
    build!(0xb1, "", (), (), (), false),
    build!(0xb2, "", (), (), (), false),
    build!(0xb3, "", (), (), (), false),
    build!(0xb4, "", (), (), (), false),
    build!(0xb5, "", (), (), (), false),
    build!(0xb6, "", (), (), (), false),
    build!(0xb7, "", (), (), (), false),
    build!(0xb8, "", (), (), (), false),
    build!(0xb9, "", (), (), (), false),
    build!(0xba, "", (), (), (), false),
    build!(0xbb, "", (), (), (), false),
    build!(0xbc, "", (), (), (), false),
    build!(0xbd, "", (), (), (), false),
    build!(0xbe, "", (), (), (), false),
    build!(0xbf, "", (), (), (), false),
    build!(0xc0, "", (), (), (), false),
    build!(0xc1, "", (), (), (), false),
    build!(0xc2, "", (), (), (), false),
    build!(0xc3, "", (), (), (), false),
    build!(0xc4, "", (), (), (), false),
    build!(0xc5, "", (), (), (), false),
    build!(0xc6, "", (), (), (), false),
    build!(0xc7, "", (), (), (), false),
    build!(0xc8, "", (), (), (), false),
    build!(0xc9, "", (), (), (), false),
    build!(0xca, "", (), (), (), false),
    build!(0xcb, "", (), (), (), false),
    build!(0xcc, "", (), (), (), false),
    build!(0xcd, "", (), (), (), false),
    build!(0xce, "", (), (), (), false),
    build!(0xcf, "", (), (), (), false),
    build!(0xd0, "", (), (), (), false),
    build!(0xd1, "", (), (), (), false),
    build!(0xd2, "", (), (), (), false),
    build!(0xd3, "", (), (), (), false),
    build!(0xd4, "", (), (), (), false),
    build!(0xd5, "", (), (), (), false),
    build!(0xd6, "", (), (), (), false),
    build!(0xd7, "", (), (), (), false),
    build!(0xd8, "", (), (), (), false),
    build!(0xd9, "", (), (), (), false),
    build!(0xda, "", (), (), (), false),
    build!(0xdb, "", (), (), (), false),
    build!(0xdc, "", (), (), (), false),
    build!(0xdd, "", (), (), (), false),
    build!(0xde, "", (), (), (), false),
    build!(0xdf, "", (), (), (), false),
    build!(0xe0, "", (), (), (), false),
    build!(0xe1, "", (), (), (), false),
    build!(0xe2, "", (), (), (), false),
    build!(0xe3, "", (), (), (), false),
    build!(0xe4, "", (), (), (), false),
    build!(0xe5, "", (), (), (), false),
    build!(0xe6, "", (), (), (), false),
    build!(0xe7, "", (), (), (), false),
    build!(0xe8, "", (), (), (), false),
    build!(0xe9, "", (), (), (), false),
    build!(0xea, "", (), (), (), false),
    build!(0xeb, "", (), (), (), false),
    build!(0xec, "", (), (), (), false),
    build!(0xed, "", (), (), (), false),
    build!(0xee, "", (), (), (), false),
    build!(0xef, "", (), (), (), false),
    build!(0xf0, "", (), (), (), false),
    build!(0xf1, "", (), (), (), false),
    build!(0xf2, "", (), (), (), false),
    build!(0xf3, "", (), (), (), false),
    build!(0xf4, "", (), (), (), false),
    build!(0xf5, "", (), (), (), false),
    build!(0xf6, "", (), (), (), false),
    build!(0xf7, "", (), (), (), false),
    build!(0xf8, "", (), (), (), false),
    build!(0xf9, "", (), (), (), false),
    build!(0xfa, "", (), (), (), false),
    build!(0xfb, "", (), (), (), false),
    build!(0xfc, "", (), (), (), false),
    build!(0xfd, "", (), (), (), false),
    build!(0xfe, "", (), (), (), false),
];

// FIXME: create hashtable or some other faster lookup method and use that.
pub(crate) fn lookup_instructions_by_mnemonic(mnemonic: &str) -> Vec<&InstructionDescriptor> {
    let mnemonic = mnemonic.to_uppercase();
    INSTRUCTION_DESCRIPTORS
        .iter()
        .filter(|i| i.mnemonic == mnemonic)
        .collect()
}

pub(crate) fn lookup_instruction_by_opcode(mnemonic: &str) -> Option<InstructionDescriptor> {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectiveAddressOperator {
    Add,
    Subtract,
    Multiply,
}

impl TryFrom<char> for EffectiveAddressOperator {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Subtract),
            '*' => Ok(Self::Multiply),
            _ => Err(Error::CannotCovertType(format!(
                "'{}' does not correspond to a valid operator",
                value
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectiveAddressOperand {
    Immediate(Immediate),
    Register(Register),
}

impl TryFrom<&NasmStr<'_>> for EffectiveAddressOperand {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        if let Ok(immediate) = Immediate::try_from(value) {
            return Ok(Self::Immediate(immediate));
        }

        if let Ok(register) = Register::try_from(value) {
            return Ok(Self::Register(register));
        }

        Err(Error::CannotParseInstruction(format!(
            "cannot parse \"{}\" into a valid effective address operand",
            value.0
        )))
    }
}

/// Represents a memory reference that is constructed out of a series of operators and operands.
/// For example:
///
/// - [EAX] = [(Add, Register(Eax))]
/// - [EAX+4*EBX] = [(Add, Register(Eax)), (Add, Immediate(4)), (Multiply, Register(Ebx))]
///
/// There cannot be more than two registers used in the formation of a valid memory address,
/// therefore this is tracked and a push will fail on the third attempt to push a register.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveAddress {
    sequence: Vec<(EffectiveAddressOperator, EffectiveAddressOperand)>,
    num_registers: u8,
}

impl EffectiveAddress {
    pub fn new() -> Self {
        Self {
            sequence: Vec::new(),
            num_registers: 0,
        }
    }

    pub fn push(
        &mut self,
        operator: EffectiveAddressOperator,
        operand: EffectiveAddressOperand,
    ) -> Result<(), Error> {
        if let EffectiveAddressOperand::Register(_) = operand {
            if self.num_registers > 2 {
                return Err(Error::CannotParseInstruction(
                    "a memory address cannot be computed from more than two registers".into(),
                ));
            }
            self.num_registers += 1;
        }
        self.sequence.push((operator, operand));
        Ok(())
    }
}

impl TryFrom<&NasmStr<'_>> for EffectiveAddress {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        let mut remainder = value.0;
        let mut chars = remainder.chars();
        if chars.nth(0).unwrap() != '[' {
            return Err(Error::CannotParseInstruction(
                "invalid effective address (must start with \"[\")".into(),
            ));
        }

        if chars.last().unwrap() != ']' {
            return Err(Error::CannotParseInstruction(
                "invalid effective address (expected \"]\" at end of operand)".into(),
            ));
        }

        if remainder.len() < 3 {
            return Err(Error::CannotParseInstruction(
                "invalid effective address (no contents)".into(),
            ));
        }

        let inner = &remainder[1..remainder.len() - 1].to_lowercase();
        let mut operator = EffectiveAddressOperator::Add;
        let mut memory_operand_sequence = EffectiveAddress::new();
        for mut token in inner.split_inclusive(&['+', '-', '*']) {
            let next_operator = if let Ok(next_operator) =
                EffectiveAddressOperator::try_from(token.chars().last().unwrap())
            {
                // Remove the trailing operand and trim since whitespace is irrelevant.
                token = token[0..token.len() - 1].trim();
                next_operator
            } else {
                // Irrelevant: this is the final iteration.
                EffectiveAddressOperator::Add
            };
            let operand = EffectiveAddressOperand::try_from(&NasmStr(token))?;
            match &operand {
                EffectiveAddressOperand::Immediate(immediate) => {
                    if operator == EffectiveAddressOperator::Multiply && immediate.parsed() > 9 {
                        return Err(Error::CannotParseInstruction(format!(
                            "invalid effective address (scale can be at most 9, was {})",
                            immediate.parsed()
                        )));
                    }
                }
                EffectiveAddressOperand::Register(register) => {
                    if operator == EffectiveAddressOperator::Subtract
                        || operator == EffectiveAddressOperator::Multiply
                    {
                        return Err(Error::CannotParseInstruction(
                            "invalid effective address (registers can only be added together)"
                                .into(),
                        ));
                    }
                    if !(register == &Register::Eax
                        || register == &Register::Ebx
                        || register == &Register::Ecx
                        || register == &Register::Edx
                        || register == &Register::Edi
                        || register == &Register::Esi
                        || register == &Register::Ebp
                        || register == &Register::Esp)
                    {
                        return Err(Error::CannotParseInstruction(
                            format!("invalid effective address (must use only valid 32-bit registers, tried to use {})", register)
                        ));
                    }
                }
            }
            memory_operand_sequence.push(operator, operand)?;
            operator = next_operator;
        }

        Ok(memory_operand_sequence)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Immediate {
    raw: String,
    parsed: u64,
}

impl Immediate {
    pub fn new(raw: &str, parsed: u64) -> Self {
        Self {
            raw: raw.into(),
            parsed,
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn parsed(&self) -> u64 {
        self.parsed
    }
}

impl TryFrom<&NasmStr<'_>> for Immediate {
    type Error = Error;

    fn try_from(value: &NasmStr) -> Result<Self, Self::Error> {
        // 200          ; decimal
        // 0200         ; still decimal - the leading 0 does not make it octal
        // 0000000200   ; valid
        // 0200d        ; explicitly decimal - d suffix
        // 0d200        ; also decimal - 0d prefex
        // 00d200       ; invalid
        // 0c8h         ; hex - h suffix, but leading 0 is required because c8h looks like a var
        // 0xc8         ; hex - the classic 0x prefix
        // 0hc8         ; hex - for some reason NASM likes 0h
        // 310q         ; octal - q suffix
        // 0q310        ; octal - 0q prefix
        // 11001000b    ; binary - b suffix
        // 0b1100_1000
        // 0d prefix/d suffix = decimal
        // 0q prefix/q suffix = octal
        // 0..h               = hex
        // ..h (where first char is numberic) = hex
        // 0x...              = hex
        // 0h...              = hex
        let parse = |value: &str, radix: u32, radix_name: &str| {
            let parsed = u64::from_str_radix(value, radix).map_err(|_| {
                Error::CannotParseInstruction(format!(
                    "could not parse {} as {}",
                    value, radix_name
                ))
            })?;
            return Ok(Immediate {
                raw: value.into(),
                parsed,
            });
        };

        if value.0.len() > 1 {
            let value_without_suffix = &value.0[..value.0.len() - 1];
            if value.0.ends_with("b") {
                return parse(value_without_suffix, 2, "binary");
            }

            if value.0.ends_with("q") {
                return parse(value_without_suffix, 8, "octal");
            }

            if value.0.ends_with("d") {
                return parse(value_without_suffix, 10, "decimal");
            }

            if value.0.chars().nth(0).unwrap().is_numeric() && value.0.ends_with("h") {
                return parse(value_without_suffix, 16, "hexadecimal");
            }
        }

        if value.0.len() > 2 {
            let prefix = value.0[0..=1].to_lowercase();
            let value_without_prefix = &value.0[2..];
            if prefix == "0b" {
                return parse(value_without_prefix, 2, "binary");
            }

            if prefix == "0q" {
                return parse(value_without_prefix, 8, "octal");
            }

            if prefix == "0d" {
                return parse(value_without_prefix, 10, "decimal");
            }

            if prefix == "0h" || prefix == "0x" {
                return parse(value_without_prefix, 16, "hexadecimal");
            }
        }

        let parsed = value.0.parse::<u64>().map_err(|_| {
            Error::CannotCovertType(format!("invalid immediate value ({})", value.0))
        })?;

        Ok(Immediate {
            raw: value.0.to_string(),
            parsed,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperandType {
    Immediate(Immediate),
    Memory(EffectiveAddress),
    Register(Register),
}

impl TryFrom<&NasmStr<'_>> for OperandType {
    type Error = Error;

    fn try_from(nasm_str: &NasmStr<'_>) -> Result<Self, Self::Error> {
        if let Ok(immediate) = Immediate::try_from(nasm_str) {
            return Ok(Self::Immediate(immediate));
        }

        if let Ok(effective_address) = EffectiveAddress::try_from(nasm_str) {
            return Ok(Self::Memory(effective_address));
        }

        if let Ok(register) = Register::try_from(nasm_str) {
            return Ok(Self::Register(register));
        }

        Err(Error::CannotCovertType(format!(
            "cannot convert {} (NASM format) into a valid operand type",
            nasm_str.0
        )))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Size {
    Byte,
    Word,
    Dword,
    Qword,
}

impl TryFrom<&NasmStr<'_>> for Size {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        use Size::*;
        match value.0.to_uppercase().as_str() {
            "BYTE" => Ok(Byte),
            "WORD" => Ok(Word),
            "DWORD" => Ok(Dword),
            "QWORD" => Ok(Qword),
            value @ _ => Err(Error::CannotParseInstruction(format!(
                "cannot convert {} into a valid size",
                value
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operand {
    size_directive: Option<Size>,
    operand_type: OperandType,
}

impl Operand {
    pub fn new(operand_type: OperandType, size_directive: Option<Size>) -> Self {
        Self {
            size_directive,
            operand_type,
        }
    }
}

impl TryFrom<&NasmStr<'_>> for Operand {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        let index = if let Some(index) = value.0.find('[') {
            Some(index)
        } else if let Some(index) = value.0.find(' ') {
            Some(index)
        } else {
            None
        };

        // FIXME: There is probably a more idiomatic and clear way to achieve this without breaking
        //        out of a named block. In short, we split on the index which we think may be
        //        separating the `size_directive` and `operand_type`, but if we can't then parse
        //        the first part of that split as a `Size`, then that means the entire thing should
        //        be parsed as an `OperandType` and we need to try again. So, in the current
        //        implementation, if we fail to parse the `Size`, then we break out and fall
        //        through to the case where we try to parse everything.
        'size_directive: {
            if let Some(index) = index {
                let Ok(size_directive) = Size::try_from(&NasmStr(&value.0[..index].trim())) else {
                break 'size_directive;
            };

                let operand_type = OperandType::try_from(&NasmStr(&value.0[index..].trim()))?;
                return Ok(Self {
                    size_directive: Some(size_directive),
                    operand_type,
                });
            }
        }

        let operand_type = OperandType::try_from(&NasmStr(&value.0.trim()))?;
        Ok(Self {
            size_directive: None,
            operand_type,
        })
    }
}

pub struct NasmStr<'a>(pub &'a str);

pub struct Instruction<'a> {
    instruction_descriptor: &'a InstructionDescriptor<'a>,
    operands: Vec<OperandType>,
}

impl<'a> TryFrom<NasmStr<'a>> for Instruction<'_> {
    type Error = Error;

    fn try_from(instruction: NasmStr) -> Result<Self, Self::Error> {
        // NasmInstructionStrParser::parse(instruction.0)
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_try_from_nasm_str() {
        assert!(Immediate::try_from(&NasmStr("00d200")).is_err());
        assert!(Immediate::try_from(&NasmStr("c0h")).is_err());

        let input = "0x200";
        let expected_parsed = 512;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "0h200";
        let expected_parsed = 512;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "000200h";
        let expected_parsed = 512;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "0d200";
        let expected_parsed = 200;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "200d";
        let expected_parsed = 200;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "0q0200";
        let expected_parsed = 128;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "000000000000000000000000000000000000000000000000000000200q";
        let expected_parsed = 128;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "0b00101";
        let expected_parsed = 5;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let input = "000000000000000000000000000000000000000000000000000000000000000000000000101b";
        let expected_parsed = 5;
        let expected_immediate = Immediate {
            raw: input.into(),
            parsed: expected_parsed,
        };
        let immediate = Immediate::try_from(&NasmStr(input)).unwrap();
        assert_eq!(immediate, expected_immediate);
    }

    #[test]
    fn operand_from_nasm_str() {
        assert!(Operand::try_from(&NasmStr("WORDEBX")).is_err());
        assert!(Operand::try_from(&NasmStr(" wordax ")).is_err());
        assert!(Operand::try_from(&NasmStr("WORD2")).is_err());
        assert!(Operand::try_from(&NasmStr("wor eax")).is_err());
    }
}
