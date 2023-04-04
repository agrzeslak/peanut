use crate::{
    cpu::Cpu,
    error::Error,
    register::{Register, Register16, Register32, Register8},
    traits::{AsUnsigned, RegisterReadWrite},
};

#[derive(Debug)]
enum InstructionOperandFormat {
    Cs,
    Ds,
    Es,
    Fs,
    Gs,
    Ss,
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
    Reg16Mem,
    Reg32Mem,
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

impl InstructionOperandFormat {
    /// Checks whether the `InstructionOperandFormat` is compatible with the operands provided.
    /// I.e. can an instruction with this `InstructionOperandFormat` be executed on the operands
    /// provided.
    pub fn matches(&self, operands: &Operands) -> bool {
        // Validates that the operand is the correct immediate value.
        let validate_const = |operand: &Operand, target: u32| -> bool {
            if let OperandType::Immediate(immediate) = &operand.operand_type {
                immediate.0 == target
            } else {
                false
            }
        };

        // Validates that the immediate operand's size directive (if given) matches the target
        // size. If no size directive is provided, then it is accepted regardless of whether it may
        // be too large. In the case that it is, it will simply be truncated when used. This
        // tangentially allows negative numbers to work as expected, as inferring the size of -1
        // would always result in `u32::MAX` due to two's complement encoding, even though it can
        // equivalently fit in a BYTE (`u8::MAX`), or WORD (`u16::MAX`).
        let validate_immediate = |operand: &Operand, target_size: Size| -> bool {
            let OperandType::Immediate(_) = &operand.operand_type else {
                    return false;
                };

            if let Some(size_directive) = &operand.size_directive {
                return size_directive == &target_size;
            }

            true
        };

        // Validates that the register contained within this operand is of the specified
        // `target_size`.
        let validate_register = |operand: &Operand, target_size: Size| -> bool {
            let OperandType::Register(register) = &operand.operand_type else {
                return false;
            };
            register.size() == target_size
        };

        // Validates that the operand containing this effective address either does not have a size
        // directive, or that it has a matching size directive. If `target_size` is `None`, then we
        // do not perform any size checks.
        let validate_memory = |operand: &Operand, target_size: Option<Size>| -> bool {
            let OperandType::Memory(_) = &operand.operand_type else {
                return false;
            };

            let Some(target_size) = target_size else {
                return true;
            };

            if let Some(size_directive) = &operand.size_directive {
                return size_directive == &target_size;
            }

            true
        };

        // Validates that either a register or effective address has been provided. If it is a
        // register, it should also be of the specified `target_size`.
        let validate_register_or_memory = |operand: &Operand, target_size: Size| -> bool {
            match &operand.operand_type {
                OperandType::Memory(_) => {
                    let Some(size_directive) = &operand.size_directive else {
                        return true;
                    };
                    size_directive == &target_size
                },
                OperandType::Register(register) => register.size() == target_size,
                _ => false,
            }
        };

        use InstructionOperandFormat as F;
        match (
            self,
            operands.0.get(0),
            operands.0.get(1),
            operands.0.get(2),
        ) {
            (F::Cs, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Cs.into())
            }
            (F::Ds, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Ds.into())
            }
            (F::Es, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Es.into())
            }
            (F::Fs, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Fs.into())
            }
            (F::Gs, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Gs.into())
            }
            (F::Ss, Some(op), None, None) => {
                op.operand_type == OperandType::Register(Register16::Ss.into())
            }
            (F::Const3, Some(op), None, None) => validate_const(op, 3),
            (F::Imm8, Some(op), None, None) => validate_immediate(op, Size::Byte),
            (F::Imm16, Some(op), None, None) => validate_immediate(op, Size::Word),
            (F::Imm32, Some(op), None, None) => validate_immediate(op, Size::Dword),
            (F::Reg16, Some(op), None, None) => validate_register(op, Size::Word),
            (F::Reg32, Some(op), None, None) => validate_register(op, Size::Dword),
            (F::Reg8Imm8, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Byte) && validate_immediate(op2, Size::Byte)
            }
            (F::Reg16Imm16, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Word) && validate_immediate(op2, Size::Word)
            }
            (F::Reg32Imm32, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Dword) && validate_immediate(op2, Size::Dword)
            }
            // (F::Rel8, Some(op), None, None) => {},
            // (F::Rel16, Some(op), None, None) => {},
            // (F::Rel32, Some(op), None, None) => {},
            (F::Rm8, Some(op), None, None) => validate_register_or_memory(op, Size::Byte),
            (F::Rm16, Some(op), None, None) => validate_register_or_memory(op, Size::Word),
            (F::Rm32, Some(op), None, None) => validate_register_or_memory(op, Size::Dword),
            (F::Reg8Rm8, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Byte) && validate_register_or_memory(op2, Size::Byte)
            }
            (F::Reg16Rm16, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Word) && validate_register_or_memory(op2, Size::Word)
            }
            (F::Reg32Rm32, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Dword) && validate_register_or_memory(op2, Size::Dword)
            }
            (F::Rm8Reg8, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Byte) && validate_register(op2, Size::Byte)
            }
            (F::Rm16Reg16, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Word) && validate_register(op2, Size::Word)
            }
            (F::Rm32Reg32, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Dword) && validate_register(op2, Size::Dword)
            }
            // (F::Rm16Sreg, Some(op), None, None) => {},
            // (F::Rm32Sreg, Some(op), None, None) => {},
            (F::Rm8Imm8, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Byte) && validate_immediate(op2, Size::Byte)
            }
            (F::Rm16Imm16, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Word) && validate_immediate(op2, Size::Word)
            }
            (F::Rm16Imm8, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Word) && validate_immediate(op2, Size::Byte)
            }
            (F::Rm32Imm8, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Dword) && validate_immediate(op2, Size::Byte)
            }
            (F::Rm32Imm32, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Dword)
                    && validate_immediate(op2, Size::Dword)
            }
            (F::Reg16Rm16Imm8, Some(op1), Some(op2), Some(op3)) => {
                validate_register(op1, Size::Word)
                    && validate_register_or_memory(op2, Size::Word)
                    && validate_immediate(op3, Size::Byte)
            }
            (F::Reg16Rm16Imm16, Some(op1), Some(op2), Some(op3)) => {
                validate_register(op1, Size::Word)
                    && validate_register_or_memory(op2, Size::Word)
                    && validate_immediate(op3, Size::Word)
            }
            (F::Reg32Rm32Imm8, Some(op1), Some(op2), Some(op3)) => {
                validate_register(op1, Size::Dword)
                    && validate_register_or_memory(op2, Size::Dword)
                    && validate_immediate(op3, Size::Byte)
            }
            (F::Reg32Rm32Imm32, Some(op1), Some(op2), Some(op3)) => {
                validate_register(op1, Size::Dword)
                    && validate_register_or_memory(op2, Size::Dword)
                    && validate_immediate(op3, Size::Dword)
            }
            (F::Reg16Mem, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Word) && validate_memory(op2, None)
            }
            (F::Reg32Mem, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Dword) && validate_memory(op2, None)
            }
            // (F::SregRm16, Some(op), None, None) => {},
            // (F::SregRm32, Some(op), None, None) => {},
            (F::Rm8Const1, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Byte) && validate_const(op2, 1)
            }
            (F::Rm16Const1, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Word) && validate_const(op2, 1)
            }
            (F::Rm32Const1, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Dword) && validate_const(op2, 1)
            }
            // (F::Far16, Some(op), None, None) => {},
            // (F::Far32, Some(op), None, None) => {},
            (F::Rm8Cl, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Byte)
                    && op2.operand_type == OperandType::Register(Register8::Cl.into())
            }
            (F::Rm16Cl, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Word)
                    && op2.operand_type == OperandType::Register(Register8::Cl.into())
            }
            (F::Rm32Cl, Some(op1), Some(op2), None) => {
                validate_register_or_memory(op1, Size::Dword)
                    && op2.operand_type == OperandType::Register(Register8::Cl.into())
            }
            // (F::Reg32Cr, Some(op1), Some(op2), None) => {},
            // (F::Reg32Dr, Some(op1), Some(op2), None) => {},
            // (F::CrReg32, Some(op1), Some(op2), None) => {},
            // (F::DrReg32, Some(op1), Some(op2), None) => {},
            (F::Reg16Rm8, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Word) && validate_register_or_memory(op2, Size::Byte)
            }
            (F::Reg32Rm8, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Dword) && validate_register_or_memory(op2, Size::Byte)
            }
            (F::Reg32Rm16, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Dword) && validate_register_or_memory(op2, Size::Word)
            }
            (F::Rm16Reg16Imm8, Some(op1), Some(op2), Some(op3)) => {
                validate_register_or_memory(op1, Size::Word)
                    && validate_register(op2, Size::Word)
                    && validate_immediate(op3, Size::Byte)
            }
            (F::Rm32Reg32Imm8, Some(op1), Some(op2), Some(op3)) => {
                validate_register_or_memory(op1, Size::Dword)
                    && validate_register(op2, Size::Dword)
                    && validate_immediate(op3, Size::Byte)
            }
            (F::Rm16Reg16Cl, Some(op1), Some(op2), Some(op3)) => {
                validate_register_or_memory(op1, Size::Word)
                    && validate_register(op2, Size::Word)
                    && op3.operand_type == OperandType::Register(Register8::Cl.into())
            }
            (F::Rm32Reg32Cl, Some(op1), Some(op2), Some(op3)) => {
                validate_register_or_memory(op1, Size::Dword)
                    && validate_register(op2, Size::Dword)
                    && op3.operand_type == OperandType::Register(Register8::Cl.into())
            }
            (F::AlImm8, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register8::Al.into())
                    && validate_immediate(op2, Size::Byte)
            }
            (F::AxImm16, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Ax.into())
                    && validate_immediate(op2, Size::Word)
            }
            (F::EaxImm32, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register32::Eax.into())
                    && validate_immediate(op2, Size::Dword)
            }
            (F::Imm16Imm16, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Word) && validate_immediate(op2, Size::Word)
            }
            (F::Imm16Imm32, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Word) && validate_immediate(op2, Size::Dword)
            }
            (F::AxReg16, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Ax.into())
                    && validate_register(op2, Size::Word)
            }
            (F::EaxReg32, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register32::Eax.into())
                    && validate_register(op2, Size::Dword)
            }
            (F::AxImm8, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Ax.into())
                    && validate_immediate(op2, Size::Byte)
            }
            (F::EaxImm8, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register32::Eax.into())
                    && validate_immediate(op2, Size::Byte)
            }
            // TODO: implement below
            // (F::AlMoffs8, Some(op1), Some(op2), None) => {},
            // (F::AxMoffs16, Some(op1), Some(op2), None) => {},
            // (F::EaxMoffs32, Some(op1), Some(op2), None) => {},
            // (F::Moffs8Al, Some(op1), Some(op2), None) => {},
            // (F::Moffs16Ax, Some(op1), Some(op2), None) => {},
            // (F::Moffs32Eax, Some(op1), Some(op2), None) => {},
            (F::AlDx, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register8::Al.into())
                    && op2.operand_type == OperandType::Register(Register16::Dx.into())
            }
            (F::AxDx, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Ax.into())
                    && op2.operand_type == OperandType::Register(Register16::Dx.into())
            }
            (F::EaxDx, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register32::Eax.into())
                    && op2.operand_type == OperandType::Register(Register16::Dx.into())
            }
            (F::DxAl, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Dx.into())
                    && op2.operand_type == OperandType::Register(Register8::Al.into())
            }
            (F::DxAx, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Dx.into())
                    && op2.operand_type == OperandType::Register(Register16::Ax.into())
            }
            (F::DxEax, Some(op1), Some(op2), None) => {
                op1.operand_type == OperandType::Register(Register16::Dx.into())
                    && op2.operand_type == OperandType::Register(Register32::Eax.into())
            }
            (F::Imm8Al, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Byte)
                    && op2.operand_type == OperandType::Register(Register8::Al.into())
            }
            (F::Imm8Ax, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Byte)
                    && op2.operand_type == OperandType::Register(Register16::Ax.into())
            }
            (F::Imm8Eax, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Byte)
                    && op2.operand_type == OperandType::Register(Register32::Eax.into())
            }
            (F::Imm8Imm16, Some(op1), Some(op2), None) => {
                validate_immediate(op1, Size::Byte) && validate_immediate(op2, Size::Word)
            }
            (F::Reg8Cl, Some(op1), Some(op2), None) => {
                validate_register(op1, Size::Byte)
                    && op2.operand_type == OperandType::Register(Register8::Cl.into())
            }
            _ => false,
        }
    }
}

type CpuFunction = fn(&mut Cpu, &Operands);

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
    // FIXME: Unsure if this should be stored here, perhaps it should just be encoded within the
    //        relevant CPU functions.
    lock_prefix: bool,
}

/// Finds the appropriate `InstructionDescriptor` based on the mnemonic and operands provided.
/// Multiple instructions may share the same menmonic, but they should be differentiated by their
/// operand sizes.
impl<'a> InstructionDescriptor<'a> {
    // FIXME: This will need to be refactored when we want to support more than just NASM, similarly
    //        to how there are different `TryInto` implementations based on what type of instruction
    //        format you are passing.
    // FIXME: Signature could be made more ergonomic by accepting a borrowed iterator in some form.
    pub fn lookup_using_mnemonic_and_operands(
        mnemonic: &str,
        operands: &Operands,
    ) -> Result<CpuFunction, Error> {
        let mnemonic = mnemonic.to_uppercase();
        let candidates: Vec<_> = INSTRUCTION_DESCRIPTORS
            .iter()
            .filter(|i| i.mnemonic == mnemonic)
            .collect();

        let mut matching_cpu_functions = Vec::new();
        for candidate in &candidates {
            if let Some(cpu_function) = candidate.resolve_matching_cpu_function(operands)? {
                matching_cpu_functions.push(cpu_function);
            }
        }

        match matching_cpu_functions.len() {
            0 => Err(Error::NoMatchingInstruction(format!("an instruction could not be found that matches the mnemonic \"{mnemonic}\" and associated operands"))),
            1 => Ok(*matching_cpu_functions.get(0).unwrap()),
            _ => Err(Error::AmbiguousInstruction(format!("the mnemonic \"{mnemonic}\" and associated operands do not uniquely match a single instruction"))),
        }
    }

    /// An `InstructionDescriptor` may have multiple `CpuFunction`, each for different operands.
    /// For a given set of operands, this function will find the appropriate `CpuFunction`, if it
    /// exists.
    pub fn resolve_matching_cpu_function(
        &self,
        operands: &Operands,
    ) -> Result<Option<CpuFunction>, Error> {
        let mut cpu_function = None;

        if let Some(map) = &self.operand_function_map_8 {
            if map.instruction_operand_format.matches(operands) {
                cpu_function = Some(map.cpu_function);
            }
        };

        if let Some(map) = &self.operand_function_map_16 {
            if map.instruction_operand_format.matches(operands) {
                if cpu_function.is_some() {
                    return Err(Error::AmbiguousInstruction(format!("ambigious operand(s)")));
                }
                cpu_function = Some(map.cpu_function);
            }
        };

        if let Some(map) = &self.operand_function_map_32 {
            if map.instruction_operand_format.matches(operands) {
                if cpu_function.is_some() {
                    return Err(Error::AmbiguousInstruction(format!("ambigious operand(s)")));
                }
                cpu_function = Some(map.cpu_function);
            }
        };

        Ok(cpu_function)
    }
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
    build!(0x2e, "CS", (), (), (), false),
    build!(0x2f, "DAS", (), (), (), false),
    build!(0x30, "XOR", (), (), (), true),
    build!(0x31, "XOR", (), (), (), true),
    build!(0x32, "XOR", (), (), (), false),
    build!(0x33, "XOR", (), (), (), false),
    build!(0x34, "XOR", (), (), (), false),
    build!(0x35, "XOR", (), (), (), false),
    build!(0x36, "SS", (), (), (), false),
    build!(0x37, "AAA", (), (), (), false),
    build!(0x38, "CMP", (), (), (), false),
    build!(0x39, "CMP", (), (), (), false),
    build!(0x3a, "CMP", (), (), (), false),
    build!(0x3b, "CMP", (), (), (), false),
    build!(0x3c, "CMP", (), (), (), false),
    build!(0x3d, "CMP", (), (), (), false),
    build!(0x3e, "DS", (), (), (), false),
    build!(0x3f, "AAS", (), (), (), false),
    build!(0x40, "INC", (), (), (), false),
    build!(0x41, "INC", (), (), (), false),
    build!(0x42, "INC", (), (), (), false),
    build!(0x43, "INC", (), (), (), false),
    build!(0x44, "INC", (), (), (), false),
    build!(0x45, "INC", (), (), (), false),
    build!(0x46, "INC", (), (), (), false),
    build!(0x47, "INC", (), (), (), false),
    build!(0x48, "DEC", (), (), (), false),
    build!(0x49, "DEC", (), (), (), false),
    build!(0x4a, "DEC", (), (), (), false),
    build!(0x4b, "DEC", (), (), (), false),
    build!(0x4c, "DEC", (), (), (), false),
    build!(0x4d, "DEC", (), (), (), false),
    build!(0x4e, "DEC", (), (), (), false),
    build!(0x4f, "DEC", (), (), (), false),
    build!(0x50, "PUSH", (), (), (), false),
    build!(0x51, "PUSH", (), (), (), false),
    build!(0x52, "PUSH", (), (), (), false),
    build!(0x53, "PUSH", (), (), (), false),
    build!(0x54, "PUSH", (), (), (), false),
    build!(0x55, "PUSH", (), (), (), false),
    build!(0x56, "PUSH", (), (), (), false),
    build!(0x57, "PUSH", (), (), (), false),
    build!(0x58, "POP", (), (), (), false),
    build!(0x59, "POP", (), (), (), false),
    build!(0x5a, "POP", (), (), (), false),
    build!(0x5b, "POP", (), (), (), false),
    build!(0x5c, "POP", (), (), (), false),
    build!(0x5d, "POP", (), (), (), false),
    build!(0x5e, "POP", (), (), (), false),
    build!(0x5f, "POP", (), (), (), false),
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
    build!(0x88, "MOV", (Rm8Reg8, mov_rm8_reg8), (), (), false),
    build!(
        0x89,
        "MOV",
        (),
        (Rm16Reg16, mov_rm16_reg16),
        (Reg32Rm32, mov_rm32_reg32),
        false
    ),
    build!(0x8a, "MOV", (Reg8Rm8, mov_reg8_rm8), (), (), false),
    build!(
        0x8b,
        "MOV",
        (),
        (Reg16Rm16, mov_reg16_rm16),
        (Reg32Rm32, mov_reg32_rm32),
        false
    ),
    build!(0x8c, "MOV", (), (), (), false),
    build!(0x8d, "LEA", (), (Reg16Mem, lea_reg16_mem), (Reg32Mem, lea_reg32_mem), false),
    build!(0x8e, "MOV", (), (), (), false),
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
// TODO: I don't understand how assemblers choose which opcode to use when multiple would match.
//       For example ADD r8, rm8 vs ADD rm8, r8. How does ADD al, bl choose which one is correct?
pub(crate) fn lookup_instructions_by_mnemonic(mnemonic: &str) -> Vec<&InstructionDescriptor> {
    let mnemonic = mnemonic.to_uppercase();
    INSTRUCTION_DESCRIPTORS
        .iter()
        .filter(|i| i.mnemonic == mnemonic)
        .collect()
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
            match register {
                Register::Register32(_) => return Ok(Self::Register(register)),
                _ => return Err(Error::CannotParseInstruction(
                    format!("invalid effective address (must use only valid 32-bit registers, tried to use {})", register)
                )),
            }
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
/// There cannot be more than two registers used in the formation of a valid effective address,
/// therefore this is tracked and a push will fail on the third attempt to push a register.
/// Registers used must be general purpose.
// FIXME: Apparently only general purpose registers should be able to be used as the base, but NASM
//        appears to also allow si, di, bp, and bx.
//        https://stackoverflow.com/questions/34058101/referencing-the-contents-of-a-memory-location-x86-addressing-modes/34058400#34058400
// TODO: Should this just be SIB?
// TODO: Tests. Also ensure that EIP cannot be used.
// TODO: Remove num_registers and register_size, which are only used during creation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveAddress {
    raw: Vec<(EffectiveAddressOperator, EffectiveAddressOperand)>,
    num_registers: u8,
    register_size: Option<Size>,
}

impl EffectiveAddress {
    pub fn new() -> Self {
        Self {
            raw: Vec::new(),
            num_registers: 0,
            register_size: None,
        }
    }

    pub fn resolve(&self, cpu: &Cpu) -> u32 {
        let mut result = 0;

        for (operator, operand) in &self.raw {
            let operand = match operand {
                EffectiveAddressOperand::Immediate(immediate) => immediate.0,
                EffectiveAddressOperand::Register(register) => match register {
                    Register::Register32(r) => r.read(&cpu.registers),
                    Register::Register16(r) => r.read(&cpu.registers).into(),
                    Register::Register8(r) => r.read(&cpu.registers).into(),
                },
            };

            match operator {
                EffectiveAddressOperator::Add => result = result + operand,
                EffectiveAddressOperator::Subtract => result = result - operand,
                EffectiveAddressOperator::Multiply => result = result * operand,
            }
        }

        result
    }

    // TODO: Tests.
    pub fn try_push(
        &mut self,
        operator: EffectiveAddressOperator,
        operand: EffectiveAddressOperand,
    ) -> Result<(), Error> {
        if let EffectiveAddressOperand::Register(register) = &operand {
            self.num_registers += 1;
            if self.num_registers > 2 {
                return Err(Error::InvalidEffectiveAddress(
                    "an effective address cannot be computed from more than two registers".into(),
                ));
            }

            if let Some(size) = &self.register_size {
                if size != &register.size() {
                    return Err(Error::InvalidEffectiveAddress(
                        "an effective address cannot be computed from two registers of different sizes".into(),
                    ));
                }
            } else {
                self.register_size = Some(register.size().clone());
            }
        }

        self.raw.push((operator, operand));
        Ok(())
    }

    // FIXME: If this can be implemented under the TryFrom trait that would be great. Am having
    //        issues with it conflicting with the core generic implementation.
    // TODO: Never used. Intuitively this should be used somewhere but I forgot to replace it.
    //       Remove otherwise.
    // TODO: Tests.
    pub fn try_from_iter<I>(iterator: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = (EffectiveAddressOperator, EffectiveAddressOperand)>,
    {
        let mut effective_address = EffectiveAddress::new();
        for (operator, operand) in iterator {
            effective_address.try_push(operator, operand)?;
        }
        Ok(effective_address)
    }
}

impl TryFrom<&NasmStr<'_>> for EffectiveAddress {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        // FIXME: This entire function is far too complex and should be simplified.
        let remainder = value.0;
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

        let inner = &remainder[1..remainder.len() - 1].trim().to_lowercase();
        let mut operator = EffectiveAddressOperator::Add;
        let mut memory_operand_sequence = EffectiveAddress::new();
        let mut first_iteration = true;
        for mut token in inner.split_inclusive(&['+', '-', '*']) {
            let next_operator = if let Ok(next_operator) =
                EffectiveAddressOperator::try_from(token.chars().last().unwrap())
            {
                // Remove the trailing operand and trim since whitespace is irrelevant.
                token = &token[0..token.len() - 1];
                next_operator
            } else {
                // Irrelevant: this is the final iteration.
                EffectiveAddressOperator::Add
            };

            // Handles the case where there is an operator at the start of the effective address
            // e.g. [+1]. In this case, the first split will be "+" and we need to keep this for
            // the next iteration and move on.
            if token.len() == 0 && first_iteration {
                if next_operator == EffectiveAddressOperator::Multiply {
                    return Err(Error::CannotParseInstruction(
                        "an effective address cannot begin with a multiplication operator".into(),
                    ));
                }
                continue;
            }

            token = token.trim();
            let operand = EffectiveAddressOperand::try_from(&NasmStr(token))?;
            match &operand {
                EffectiveAddressOperand::Immediate(immediate) => {
                    if operator == EffectiveAddressOperator::Multiply && immediate.0 > 9 {
                        return Err(Error::CannotParseInstruction(format!(
                            "invalid effective address (scale can be at most 9, was {})",
                            immediate.0
                        )));
                    }
                }
                EffectiveAddressOperand::Register(_) => {
                    if operator == EffectiveAddressOperator::Subtract
                        || operator == EffectiveAddressOperator::Multiply
                    {
                        return Err(Error::CannotParseInstruction(
                            "invalid effective address (registers can only be added together)"
                                .into(),
                        ));
                    }
                }
            }
            memory_operand_sequence.try_push(operator, operand)?;
            operator = next_operator;
            first_iteration = false;
        }

        Ok(memory_operand_sequence)
    }
}

impl<'a> TryFrom<&'a OperandType> for &'a EffectiveAddress {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(_) => Err(Error::CannotCovertType(
                "an immediate was provided when a memory reference was expected".into(),
            )),
            OperandType::Memory(effective_address) => Ok(effective_address),
            OperandType::Register(_) => Err(Error::CannotCovertType(
                "a register was provided when a memory reference was expected".into(),
            )),
        }
    }
}

// FIXME: Should likely have Immediate8/16/32 variants in order to enforce sizes when executing
//        instructions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Immediate(pub u32);

impl Immediate {
    pub fn infer_size(&self) -> Size {
        const BYTE_LOW: u32 = u8::MIN as u32;
        const BYTE_HIGH: u32 = u8::MAX as u32;

        const WORD_LOW: u32 = u16::MIN as u32;
        const WORD_HIGH: u32 = u16::MAX as u32;

        const DWORD_LOW: u32 = u32::MIN;
        const DWORD_HIGH: u32 = u32::MAX;

        use Size::*;
        match self.0 {
            BYTE_LOW..=BYTE_HIGH => Byte,
            WORD_LOW..=WORD_HIGH => Word,
            DWORD_LOW..=DWORD_HIGH => Dword,
        }
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
        let parse = |trimmed_value: &str, radix: u32, radix_name: &str| {
            let parsed = u32::from_str_radix(trimmed_value, radix).map_err(|_| {
                Error::CannotParseInstruction(format!(
                    "could not parse {} as {}",
                    trimmed_value, radix_name
                ))
            })?;
            return Ok(Immediate(parsed));
        };

        let to_parse = value.0.replace('_', "");

        if to_parse.len() > 1 {
            let value_without_suffix = &to_parse[..to_parse.len() - 1];
            if to_parse.ends_with("b") {
                return parse(value_without_suffix, 2, "binary");
            }

            if to_parse.ends_with("q") {
                return parse(value_without_suffix, 8, "octal");
            }

            if to_parse.ends_with("d") {
                return parse(value_without_suffix, 10, "decimal");
            }

            if to_parse.chars().nth(0).unwrap().is_numeric() && to_parse.ends_with("h") {
                return parse(value_without_suffix, 16, "hexadecimal");
            }
        }

        if to_parse.len() > 2 {
            let prefix = to_parse[0..=1].to_lowercase();
            let value_without_prefix = &to_parse[2..];
            match prefix.as_str() {
                "0b" => return parse(value_without_prefix, 2, "binary"),
                "0q" => return parse(value_without_prefix, 8, "octal"),
                "0d" => return parse(value_without_prefix, 10, "decimal"),
                "0h" | "0x" => return parse(value_without_prefix, 16, "hexadecimal"),
                _ => (),
            }
        }

        // We don't directly parse into `u32` because that would cause negative values to be
        // invalid. We therefore need to first parse into an `i64` to preserve the full range of
        // values possible, and then convert it to be unsigned, before then finally cast it to
        // `u32`. I.e. an input of -1 should result in the maximum unsigned value.
        // FIXME: Avoid going via `i64`.
        let parsed = to_parse.parse::<i64>().map_err(|_| {
            Error::CannotParseInstruction(format!("cannot parse {} as i64", to_parse))
        })?;

        let parsed = parsed.as_unsigned() as u32;

        Ok(Immediate(parsed))
    }
}

impl<'a> TryFrom<&'a OperandType> for &'a Immediate {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(immediate) => Ok(immediate),
            OperandType::Memory(_) => Err(Error::CannotCovertType(
                "a memory reference was provided when an immediate value was expected".into(),
            )),
            OperandType::Register(_) => Err(Error::CannotCovertType(
                "a register was provided when an immediate value was expected".into(),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperandType {
    Immediate(Immediate),
    Memory(EffectiveAddress),
    Register(Register),
}

impl OperandType {
    pub fn unwrap_immediate(&self) -> &Immediate {
        let Self::Immediate(immediate) = self else {
            panic!("attempted to unwrap a non-immediate variant as an immediate");
        };
        immediate
    }

    pub fn unwrap_effective_address(&self) -> &EffectiveAddress {
        let Self::Memory(effective_address) = self else {
            panic!("attempted to unwrap a non-effective address variant as an effective address");
        };
        effective_address
    }

    pub fn unwrap_register(&self) -> &Register {
        let Self::Register(register) = self else {
            panic!("attempted to unwrap a non-register variant as an register");
        };
        register
    }
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

        Err(Error::CannotParseInstruction(format!(
            "cannot convert \"{}\" (NASM format) into a valid operand type",
            nasm_str.0
        )))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size {
    Byte = 8,
    Word = 16,
    Dword = 32,
    Qword = 64,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operand {
    pub(crate) operand_type: OperandType,
    pub(crate) size_directive: Option<Size>,
}

impl Operand {
    pub fn new(operand_type: OperandType, size_directive: Option<Size>) -> Self {
        Self {
            operand_type,
            size_directive,
        }
    }
}

impl TryFrom<&NasmStr<'_>> for Operand {
    type Error = Error;

    fn try_from(value: &NasmStr<'_>) -> Result<Self, Self::Error> {
        let mut index = if let Some(index) = value.0.find('[') {
            index
        } else if let Some(index) = value.0.find(' ') {
            index
        } else {
            0
        };

        let minimum_size_directive_length = 4;
        let mut size_directive = if index >= minimum_size_directive_length {
            Size::try_from(&NasmStr(&value.0[..index].trim())).ok()
        } else {
            None
        };

        if size_directive.is_none() {
            index = 0;
        }

        let operand_type = OperandType::try_from(&NasmStr(&value.0[index..].trim()))?;
        if let Some(size) = &size_directive {
            if let OperandType::Register(register) = &operand_type {
                if size != &register.size() {
                    // Size directive does not match register size. NASM ignores the size directive
                    // in this case.
                    size_directive = None;
                }
            }
        }

        Ok(Self {
            operand_type,
            size_directive,
        })
    }
}

#[derive(Debug)]
pub struct NasmStr<'a>(pub &'a str);

pub struct Instruction {
    pub mnemonic: String,
    pub operands: Operands,
    pub cpu_function: CpuFunction,
}

pub struct Operands(pub Vec<Operand>);

impl Operands {
    /// Unwrap the operand at the given index as an `Immediate`, otherwise panic.
    pub(crate) fn unwrap_immediate(&self, index: usize) -> &Immediate {
        &self.0.get(index).unwrap().operand_type.unwrap_immediate()
    }

    /// Unwrap the operand at the given index as an `EffectiveAddress`, otherwise panic.
    pub(crate) fn unwrap_effective_address(&self, index: usize) -> &EffectiveAddress {
        &self
            .0
            .get(index)
            .unwrap()
            .operand_type
            .unwrap_effective_address()
    }

    /// Unwrap the operand at the given index as a `Register`, otherwise panic.
    pub(crate) fn unwrap_register(&self, index: usize) -> &Register {
        &self.0.get(index).unwrap().operand_type.unwrap_register()
    }
}

impl From<Vec<Operand>> for Operands {
    fn from(operands: Vec<Operand>) -> Self {
        Self(operands)
    }
}

macro_rules! unwrap_operands {
    ($operands:ident, $type1:ty) => {
        <$type1>::try_from(&$operands.0[0].operand_type).unwrap()
    };
    ($operands:ident, $type1:ty, $type2:ty) => {
        (
            <$type1>::try_from(&$operands.0[0].operand_type).unwrap(),
            <$type2>::try_from(&$operands.0[1].operand_type).unwrap(),
        )
    };
    ($operands:ident, $type1:ty, $type2:ty, $type3:ty) => {
        (
            <$type1>::try_from(&$operands.0[0].operand_type).unwrap(),
            <$type2>::try_from(&$operands.0[1].operand_type).unwrap(),
            <$type3>::try_from(&$operands.0[2].operand_type).unwrap(),
        )
    };
    ($operands:ident, $type1:ty, $type2:ty, $type3:ty, $type4:ty) => {
        (
            <$type1>::try_from(&$operands.0[0].operand_type).unwrap(),
            <$type2>::try_from(&$operands.0[1].operand_type).unwrap(),
            <$type3>::try_from(&$operands.0[2].operand_type).unwrap(),
            <$type4>::try_from(&$operands.0[3].operand_type).unwrap(),
        )
    };
}
pub(crate) use unwrap_operands;

impl<'a> TryFrom<&NasmStr<'a>> for Instruction {
    type Error = Error;

    fn try_from(instruction: &NasmStr) -> Result<Self, Self::Error> {
        let (mnemonic, remainder) =
            instruction
                .0
                .split_once(" ")
                .ok_or(Error::CannotParseInstruction(
                    "no mnemonic available".into(),
                ))?;

        let operands: Vec<_> = remainder
            .trim()
            .split(",")
            .map(|o| Operand::try_from(&NasmStr(o.trim())))
            .collect::<Result<_, _>>()?;
        let operands = Operands(operands);

        let cpu_function =
            InstructionDescriptor::lookup_using_mnemonic_and_operands(mnemonic, &operands)?;

        Ok(Self {
            mnemonic: mnemonic.into(),
            operands,
            cpu_function,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RegisterOrMemory32<'a> {
    Register(&'a Register32),
    Memory(&'a EffectiveAddress),
}

// TODO: test
impl RegisterOrMemory32<'_> {
    pub fn read(&self, cpu: &Cpu) -> Result<u32, Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.read32(register)),
            Self::Memory(effective_address) => cpu.memory.read32(effective_address.resolve(cpu)),
        }
    }

    pub fn write(&self, cpu: &mut Cpu, value: u32) -> Result<(), Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.write32(register, value)),
            Self::Memory(effective_address) => {
                cpu.memory.write32(effective_address.resolve(cpu), value)
            }
        }
    }
}

impl<'a> From<&'a Register32> for RegisterOrMemory32<'a> {
    fn from(register: &'a Register32) -> Self {
        register.into()
    }
}

impl<'a> From<&'a EffectiveAddress> for RegisterOrMemory32<'a> {
    fn from(effective_address: &'a EffectiveAddress) -> Self {
        Self::Memory(effective_address)
    }
}

impl<'a> TryFrom<&'a OperandType> for RegisterOrMemory32<'a> {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(_) => Err(Error::CannotCovertType(
                "cannot convert an immediate value into a RegisterOrMemory32".into(),
            )),
            OperandType::Memory(effective_address) => Ok(Self::Memory(effective_address)),
            OperandType::Register(register) => {
                Ok(Self::Register(<&Register32>::try_from(register)?))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RegisterOrMemory16<'a> {
    Register(&'a Register16),
    Memory(&'a EffectiveAddress),
}

// TODO: test
impl RegisterOrMemory16<'_> {
    pub fn read(&self, cpu: &Cpu) -> Result<u16, Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.read16(register)),
            Self::Memory(effective_address) => cpu.memory.read16(effective_address.resolve(cpu)),
        }
    }

    pub fn write(&self, cpu: &mut Cpu, value: u16) -> Result<(), Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.write16(register, value)),
            Self::Memory(effective_address) => {
                cpu.memory.write16(effective_address.resolve(cpu), value)
            }
        }
    }
}

impl<'a> From<&'a Register16> for RegisterOrMemory16<'a> {
    fn from(register: &'a Register16) -> Self {
        Self::Register(register)
    }
}

impl<'a> From<&'a EffectiveAddress> for RegisterOrMemory16<'a> {
    fn from(effective_address: &'a EffectiveAddress) -> Self {
        effective_address.into()
    }
}

impl<'a> TryFrom<&'a OperandType> for RegisterOrMemory16<'a> {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(_) => Err(Error::CannotCovertType(
                "cannot convert an immediate value into a RegisterOrMemory16".into(),
            )),
            OperandType::Memory(effective_address) => Ok(Self::Memory(effective_address)),
            OperandType::Register(register) => {
                Ok(Self::Register(<&Register16>::try_from(register)?))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RegisterOrMemory8<'a> {
    Register(&'a Register8),
    Memory(&'a EffectiveAddress),
}

// TODO: test
impl RegisterOrMemory8<'_> {
    pub fn read(&self, cpu: &Cpu) -> Result<u8, Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.read8(register)),
            Self::Memory(effective_address) => cpu.memory.read8(effective_address.resolve(cpu)),
        }
    }

    pub fn write(&self, cpu: &mut Cpu, value: u8) -> Result<(), Error> {
        match self {
            Self::Register(register) => Ok(cpu.registers.write8(register, value)),
            Self::Memory(effective_address) => {
                cpu.memory.write8(effective_address.resolve(cpu), value)
            }
        }
    }
}

impl<'a> From<&'a Register8> for RegisterOrMemory8<'a> {
    fn from(register: &'a Register8) -> Self {
        Self::Register(register)
    }
}

impl From<&EffectiveAddress> for RegisterOrMemory8<'_> {
    fn from(effective_address: &EffectiveAddress) -> Self {
        effective_address.into()
    }
}

impl<'a> TryFrom<&'a OperandType> for RegisterOrMemory8<'a> {
    type Error = Error;

    fn try_from(operand_type: &'a OperandType) -> Result<Self, Self::Error> {
        match operand_type {
            OperandType::Immediate(_) => Err(Error::CannotCovertType(
                "cannot convert an immediate value into a RegisterOrMemory8".into(),
            )),
            OperandType::Memory(effective_address) => Ok(Self::Memory(effective_address)),
            OperandType::Register(register) => {
                Ok(Self::Register(<&Register8>::try_from(register)?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_operand_format_matches() {
        use InstructionOperandFormat as F;

        // macro_rules! matches {
        //     ($format:ident allows $operand:tt, $($remainder:tt)*,) => {
        //         assert!(InstructionOperandFormat::$format.matches)
        //     }
        //     ($format:ident allows $($operand:tt)+,) => {
        //     };
        // }

        assert!(F::Cs.matches(&vec![Operand::try_from(&NasmStr("Cs")).unwrap()].into()));
        assert!(!F::Cs.matches(&vec![Operand::try_from(&NasmStr("Ds")).unwrap()].into()));
        // F::Es,
        // F::Fs,
        // F::Gs,
        // F::Ss,
        assert!(F::Const3.matches(&vec![Operand::try_from(&NasmStr("3")).unwrap()].into()));
        assert!(F::Const3.matches(&vec![Operand::try_from(&NasmStr("WORD 3")).unwrap()].into()));
        assert!(!F::Const3.matches(&vec![Operand::try_from(&NasmStr("4")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("0")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("1")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("byte 1")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("-1")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("byte -1")).unwrap()].into()));
        assert!(F::Imm8.matches(&vec![Operand::try_from(&NasmStr("256")).unwrap()].into()));
        assert!(!F::Imm8.matches(&vec![Operand::try_from(&NasmStr("dword 1")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("0")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("1")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("-1")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("256")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("65535")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("word 65535")).unwrap()].into()));
        assert!(F::Imm16.matches(&vec![Operand::try_from(&NasmStr("65536")).unwrap()].into()));
        assert!(!F::Imm16.matches(&vec![Operand::try_from(&NasmStr("dword 1")).unwrap()].into()));
        assert!(!F::Imm16.matches(&vec![Operand::try_from(&NasmStr("qword 1")).unwrap()].into()));
        assert!(!F::Imm16.matches(&vec![Operand::try_from(&NasmStr("[eax]")).unwrap()].into()));
        assert!(!F::Imm16.matches(&vec![Operand::try_from(&NasmStr("eax")).unwrap()].into()));
        assert!(F::Imm32.matches(&vec![Operand::try_from(&NasmStr("0")).unwrap()].into()));
        assert!(F::Imm32.matches(&vec![Operand::try_from(&NasmStr("1")).unwrap()].into()));
        assert!(F::Imm32.matches(&vec![Operand::try_from(&NasmStr("-1")).unwrap()].into()));
        // F::Reg16,
        // F::Reg32,
        // F::Reg8Imm8,
        // F::Reg16Imm16,
        // F::Reg32Imm32,
        // F::Rel8,
        // F::Rel16,
        // F::Rel32,
        // F::Rm8,
        // F::Rm16,
        // F::Rm32,
        // F::Reg8Rm8,
        // F::Reg16Rm16,
        // F::Reg32Rm32,
        // F::Rm8Reg8,
        // F::Rm16Reg16,
        // F::Rm32Reg32,
        // F::Rm16Sreg,
        // F::Rm32Sreg,
        // F::Rm8Imm8,
        // F::Rm16Imm16,
        // F::Rm16Imm8,
        // F::Rm32Imm8,
        // F::Rm32Imm32,
        // F::Reg16Rm16Imm8,
        // F::Reg16Rm16Imm16,
        // F::Reg32Rm32Imm8,
        // F::Reg32Rm32Imm32,
        // F::Reg16Mem16,
        // F::Reg32Mem32,
        // F::SregRm16,
        // F::SregRm32,
        // F::Rm8Const1,
        // F::Rm16Const1,
        // F::Rm32Const1,
        // F::Far16,
        // F::Far32,
        // F::Rm8Cl,
        // F::Rm16Cl,
        // F::Rm32Cl,
        // F::Reg32Cr,
        // F::Reg32Dr,
        // F::CrReg32,
        // F::DrReg32,
        // F::Reg16Rm8,
        // F::Reg32Rm8,
        // F::Reg32Rm16,
        // F::Rm16Reg16Imm8,
        // F::Rm32Reg32Imm8,
        // F::Rm16Reg16Cl,
        // F::Rm32Reg32Cl,
        // F::AlImm8,
        // F::AxImm16,
        // F::EaxImm32,
        // F::Imm16Imm16,
        // F::Imm16Imm32,
        // F::AxReg16,
        // F::EaxReg32,
        // F::AxImm8,
        // F::EaxImm8,
        // F::AlMoffs8,
        // F::AxMoffs16,
        // F::EaxMoffs32,
        // F::Moffs8Al,
        // F::Moffs16Ax,
        // F::Moffs32Eax,
        // F::AlDx,
        // F::AxDx,
        // F::EaxDx,
        // F::DxAl,
        // F::DxAx,
        // F::DxEax,
        // F::Imm8Al,
        // F::Imm8Ax,
        // F::Imm8Eax,
        // F::Imm8Imm16,
        // F::Reg8Cl,
        // F::None,
    }

    #[test]
    fn effective_address_operator_try_from_char() {
        assert!(EffectiveAddressOperator::try_from('/').is_err());
        assert!(EffectiveAddressOperator::try_from('&').is_err());
        assert_eq!(
            EffectiveAddressOperator::try_from('+').unwrap(),
            EffectiveAddressOperator::Add
        );
        assert_eq!(
            EffectiveAddressOperator::try_from('-').unwrap(),
            EffectiveAddressOperator::Subtract
        );
        assert_eq!(
            EffectiveAddressOperator::try_from('*').unwrap(),
            EffectiveAddressOperator::Multiply
        );
    }

    macro_rules! eao {
        (imm $value:literal) => {
            EffectiveAddressOperand::Immediate(Immediate::try_from(&NasmStr($value)).unwrap())
        };
        (reg $value:literal) => {
            EffectiveAddressOperand::Register(Register::try_from(&NasmStr($value)).unwrap())
        };
        ($value:literal) => {
            EffectiveAddressOperand::try_from(&NasmStr($value)).unwrap()
        };
    }

    macro_rules! assert_eao_err {
        ($value:literal) => {
            assert!(EffectiveAddressOperand::try_from(&NasmStr($value)).is_err())
        };
    }

    macro_rules! assert_eao_imm {
        ($value:literal) => {
            let expected = eao!(imm $value);
            let actual = eao!($value);
            assert_eq!(expected, actual)
        };
    }

    macro_rules! assert_eao_reg {
        ($value:literal) => {
            let expected = eao!(reg $value);
            let actual = eao!($value);
            assert_eq!(expected, actual)
        };
    }

    #[test]
    fn effective_address_operand_try_from_nasm_str() {
        assert_eao_err!(" 1");
        assert_eao_err!(" 1");
        assert_eao_err!("1 ");
        assert_eao_err!("[1]");
        assert_eao_err!("*1");
        assert_eao_err!("/1");
        assert_eao_err!("1+eax");
        assert_eao_err!("eax+1");
        assert_eao_err!("1+1");
        assert_eao_err!("eax+ebx");
        assert_eao_err!("ax");
        assert_eao_err!("al");

        assert_eao_imm!("+1");
        assert_eao_imm!("1");
        assert_eao_imm!("-1");

        assert_eao_reg!("eax");
    }

    macro_rules! assert_ea_err {
        ($value:literal) => {
            assert!(EffectiveAddress::try_from(&NasmStr($value)).is_err());
        };
    }

    macro_rules! ea {
        ($value:literal) => {
            EffectiveAddress::try_from(&NasmStr($value)).unwrap()
        };
    }

    #[test]
    fn effective_address_try_from_nasm_str() {
        use EffectiveAddressOperator::*;

        assert_ea_err!("1");
        assert_ea_err!("0x100");
        assert_ea_err!("a[eax]");
        assert_ea_err!("[eax]a");
        assert_ea_err!("[eax");
        assert_ea_err!("eax]");
        assert_ea_err!(" [eax] ");
        assert_ea_err!("[eax+ebx+ecx]");
        assert_ea_err!("[eax+ax]");
        assert_ea_err!("[ax+al]");
        assert_ea_err!("[ah+al]");
        assert_ea_err!("[ax]");
        assert_ea_err!("[eax-ebx]");
        assert_ea_err!("[eax*10]");
        assert_ea_err!("[eax/10]");
        assert_ea_err!("[eflags]");
        assert_ea_err!("[eip]");

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(imm "1"))],
            num_registers: 0,
            register_size: None,
        };
        assert_eq!(ea!("[1]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(imm "1"))],
            num_registers: 0,
            register_size: None,
        };
        assert_eq!(ea!("[+1]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(reg "eax"))],
            num_registers: 1,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[eax]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(reg "eax"))],
            num_registers: 1,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[     eAx     ]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(reg "eax")), (Add, eao!(reg "ebx"))],
            num_registers: 2,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[eax+ebx]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(reg "eax")), (Add, eao!(imm "4"))],
            num_registers: 1,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[ eax   +  4 ]"), expected);

        let expected = EffectiveAddress {
            raw: vec![(Add, eao!(reg "eax")), (Subtract, eao!(imm "10"))],
            num_registers: 1,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[eax-10]"), expected);

        let expected = EffectiveAddress {
            raw: vec![
                (Add, eao!(imm "8")),
                (Multiply, eao!(imm "4")),
                (Add, eao!(reg "ebx")),
            ],
            num_registers: 1,
            register_size: Some(Size::Dword),
        };
        assert_eq!(ea!("[8*4+ebx]"), expected);

        let expected = EffectiveAddress {
            raw: vec![
                (Add, eao!(reg "eax")),
                (Multiply, eao!(imm "2")),
                (Add, eao!(imm "4000q")),
                (Add, eao!(imm "2000h")),
                (Multiply, eao!(imm "8")),
                (Add, eao!(imm "0x8000")),
                (Add, eao!(imm "10d")),
                (Add, eao!(imm "020d")),
                (Add, eao!(reg "ebx")),
                (Multiply, eao!(imm "0b1")),
            ],
            num_registers: 2,
            register_size: Some(Size::Dword),
        };
        assert_eq!(
            ea!("[eax*2+4000q+2000h*8+0x8000+10d+020d+ebx*0b1]"),
            expected
        );
    }

    #[test]
    fn immediate_try_from_nasm_str() {
        assert!(Immediate::try_from(&NasmStr("00d200")).is_err());
        assert!(Immediate::try_from(&NasmStr("c0h")).is_err());
        assert!(Immediate::try_from(&NasmStr(" 1 ")).is_err());
        assert!(Immediate::try_from(&NasmStr("0q200h")).is_err());

        let to_parse = "0x200";
        let expected_parsed = 512;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0h200";
        let expected_parsed = 512;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "000200h";
        let expected_parsed = 512;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0d200h";
        let expected_parsed = 53760;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0d200";
        let expected_parsed = 200;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "200d";
        let expected_parsed = 200;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0q0200";
        let expected_parsed = 128;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0b1100_1000";
        let expected_parsed = 200;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0b11_100";
        let expected_parsed = 28;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "000000000000000000000000000000000000000000000000000000200q";
        let expected_parsed = 128;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse = "0b00101";
        let expected_parsed = 5;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);

        let to_parse =
            "000000000000000000000000000000000000000000000000000000000000000000000000101b";
        let expected_parsed = 5;
        let expected_immediate = Immediate(expected_parsed);
        let immediate = Immediate::try_from(&NasmStr(to_parse)).unwrap();
        assert_eq!(immediate, expected_immediate);
    }

    macro_rules! ot {
        (imm $value:literal) => {
            OperandType::Immediate(Immediate::try_from(&NasmStr($value)).unwrap())
        };
        (mem $value:literal) => {
            OperandType::Memory(EffectiveAddress::try_from(&NasmStr($value)).unwrap())
        };
        (reg $value:literal) => {
            OperandType::Register(Register::try_from(&NasmStr($value)).unwrap())
        };
    }

    macro_rules! assert_o_err {
        ($value:literal) => {
            assert!(Operand::try_from(&NasmStr($value)).is_err())
        };
    }

    macro_rules! o {
        ($value:literal) => {
            Operand::try_from(&NasmStr($value)).unwrap()
        };
    }

    #[test]
    fn operand_try_from_nasm_str() {
        assert_o_err!("WORDEBX");
        assert_o_err!(" wordax ");
        assert_o_err!("word [ax]");
        assert_o_err!("WORD2");
        assert_o_err!("wor eax");

        let expected = Operand::new(ot!(mem "[EAX]"), Some(Size::Dword));
        assert_eq!(o!(" DWORD[EAX]"), expected);

        let expected = Operand::new(ot!(imm "32"), Some(Size::Dword));
        assert_eq!(o!("dWoRd 32"), expected);

        let expected = Operand::new(ot!(reg "eax"), None);
        assert_eq!(o!("byte EAX"), expected);

        let expected = Operand::new(ot!(mem "[EAX+EBX*4+0x10]"), Some(Size::Qword));
        assert_eq!(o!("    qWORd     [EAX+EBX*4+0x10]"), expected);
    }

    macro_rules! assert_size_err {
        ($value:literal) => {
            assert!(Size::try_from(&NasmStr($value)).is_err())
        };
    }

    macro_rules! size {
        ($value:literal) => {
            Size::try_from(&NasmStr($value)).unwrap()
        };
    }

    #[test]
    fn size_try_from_nasm_str() {
        assert_size_err!(" byte");
        assert_size_err!("byte ");
        assert_size_err!("by te");

        assert_eq!(size!("bYtE"), Size::Byte);
        assert_eq!(size!("WORD"), Size::Word);
        assert_eq!(size!("dword"), Size::Dword);
        assert_eq!(size!("QworD"), Size::Qword);
    }

    #[test]
    fn instruction_try_from_nasm_str() {
        // TODO
    }

    #[test]
    fn immediate_infer_size() {
        assert_eq!(Immediate(0).infer_size(), Size::Byte);
        assert_eq!(Immediate(u8::MAX as u32).infer_size(), Size::Byte);
        assert_eq!(Immediate(u8::MAX as u32 + 1).infer_size(), Size::Word);
        assert_eq!(Immediate(u16::MAX as u32).infer_size(), Size::Word);
        assert_eq!(Immediate(u16::MAX as u32 + 1).infer_size(), Size::Dword);
        assert_eq!(Immediate(u32::MAX).infer_size(), Size::Dword);
    }
}
