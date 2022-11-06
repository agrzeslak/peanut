use crate::{
    error::Error,
    instruction::{
        self, Instruction, MemoryOperandOperator, MemoryOperandSequence, MemoryOperandType, Operand,
    },
    register::Register,
};

pub enum Syntax {
    Att,
    Intel,
}

pub struct AttInstructionStrParser<'a> {
    remainder: &'a str,
}

pub struct AttOperandStrParser<'a> {
    remainder: &'a str,
}

pub struct IntelInstructionStrParser<'a> {
    remainder: &'a str,
}

impl<'a> IntelInstructionStrParser<'a> {
    pub fn parse<'b>(instruction: &'a str) -> Result<Instruction<'b>, Error> {
        // https://www.cs.virginia.edu/~evans/cs216/guides/x86.html
        // Addressing Memory
        // Valid:
        // mov eax, [ebx]           Move 4 bytes in memory at address contained in EBX to EAX.
        // mov [var], ebx           Move contents of EBX into the 4 bytes at memory address var
        // mov eax, [esi-4]         Move 4 bytes at memory address ESI-4 into EAX
        // mov [esi+eax], cl        Move the contents of CL into the byte at address ESI+EAX
        // mov edx, [esi+4*ebx]     Move the 4 bytes of data at address ESI+4*EBX into EDX
        //
        // Invalid:
        // mov eax, [ebx-ecx]       Can only ADD register values
        // mov [eax+esi+edi], ebx   At most 2 registers in address computation
        //
        // ---
        //
        // Size Directives
        // - Generally size of data at given address can be inferred from assembly code instruction
        // - I.e. if loading into 32-bit register, memory is inferred to be 4 bytes wide.
        // - However, some ar ambiguous: mov [ebx], 2.
        //   - Should you move 2 as a single byte into EBX, or as a 4 byte DWORD?
        //   - Assembler must be explicitly told in this case.
        // mov BYTE PTR [ebx], 2    Move 2 into the single byte at the address stored in EBX
        // mov WORD PTR [ebx], 2    Move 16-bit int into the 2 bytes starting at address in EBX
        // mov DWORD PTR [ebx], 2   Move 32-bit int into the 4 bytes starting at address in EBX
        //
        // ---
        //
        // Instructions
        //
        // <reg32>  Any 32-bit reg (EAX, EBX, ECX, EDX, ESI, EDI, ESP, or EBP)
        // <reg16>  Any 16-bit reg (AX, BX, CX, or DX) - should also have sregs?
        // <reg8>   Any 8-bit reg (AH, BH, CH, DH, AL, BL, CL, DL)
        // <reg>    Any reg
        // <mem>    A memory address (e.g. [eax], [var + 4], or dword ptr [eax+ebx])
        // <con32>  Any 32-bit constant
        // <con16>  Any 16-bit constant
        // <con8>   Any 8-bit constant
        // <con>    Any 8-, 16-, or 32-bit constant
        let mut parser = Self {
            remainder: instruction.trim(),
        };

        let mnemonic = parser.parse_mnemonic()?;

        let instruction_descriptors = instruction::lookup_instructions_by_mnemonic(mnemonic);
        if instruction_descriptors.is_empty() {
            return Err(Error::CannotParseInstruction(
                "no matching mnemonics".into(),
            ));
        }

        let operands = parser.parse_operands();
        // Try match these operands against any of the matching instruction descriptors. If none
        // match it's a parsing error. If multiple match throw an ambiguity error. Unsure if we
        // should be more permissive but I don't think so.
        todo!()
    }

    fn parse_mnemonic(&mut self) -> Result<&'a str, Error> {
        let (mnemonic, remainder) =
            self.remainder
                .split_once(" ")
                .ok_or(Error::CannotParseInstruction(
                    "no mnemonic available".into(),
                ))?;
        self.remainder = remainder.trim();
        Ok(mnemonic)
    }

    fn parse_operands(&mut self) -> Result<Vec<Operand>, Error> {
        self.remainder
            .split(",")
            .map(|o| IntelOperandStrParser::parse(o.trim()))
            .collect()
    }
}

// TODO: Remove full, is never used.
pub struct IntelOperandStrParser<'a> {
    remainder: &'a str,
}

impl<'a> IntelOperandStrParser<'a> {
    fn parse(operand: &'a str) -> Result<Operand, Error> {
        // Valid:
        //     BYTE PTR [ebx]
        //     2
        //     ebx
        //     [ebx]
        //     [ebx-1]
        //     [ebx+1]
        //     [ebx+eax]
        //     [ebx+4*eax]
        //     [eax*4+0x419260]
        // Invalid:
        //     [ebx-eax] no subtraction with registers.
        //     [ebx+eax+ecx] at most 2 registers.
        // TODO: Check if [ebx+eax*4] is invalid (imm after reg in offset).
        // TODO: Check if [ebx + eax] is invalid (spaces around operator).
        // TODO: Eventually will need to parse variable names that refer to variables within the
        //       assembly file.
        // TODO: Check if you can add different sized registers together e.g. [eax+bx]. I suspect
        //       not. Add test cases accordingly.
        let mut parser = Self {
            remainder: operand.trim(),
        };

        let empty_error =
            Error::CannotParseInstruction("ran out of tokens while parsing operand".into());
        if parser.remainder.is_empty() {
            return Err(empty_error);
        }

        // Helper functions:
        // Check if there are any brackets -> memory.
        // Check if all numeric (including 0x...) -> immediate (can also help resolving immediates
        // within memory address references).

        let size_directive = parser.parse_size_directive()?;
        if parser.remainder.is_empty() {
            return Err(empty_error);
        }

        // 2. Is it a memory reference?
        let memory_reference = parser.parse_memory_reference()?;
        if memory_reference.is_none() && size_directive.is_some() {
            return Err(Error::CannotParseInstruction(
                "a size directive was provided, but no memory address".into(),
            ));
        }

        // 3. Is it a register?
        // 4. Is it an immediate value?

        todo!()
    }

    fn parse_size_directive(&mut self) -> Result<Option<u8>, Error> {
        let remainder_at_start = self.remainder;

        let (token, remainder) = self
            .remainder
            .split_once(" ")
            .ok_or(Error::CannotParseInstruction("operand is empty".into()))?;
        self.remainder = remainder;

        let size_directive = match token.to_uppercase().as_str() {
            "BYTE" => Some(1u8),
            "WORD" => Some(2u8),
            "DWORD" => Some(4u8),
            "QWORD" => Some(8u8),
            _ => None,
        };

        // If a size directive was provided, the next token should be "PTR".
        if size_directive.is_some() {
            let error =
                Error::CannotParseInstruction("operand size specifier is incomplete".into());
            let (token, remainder) = remainder.split_once(" ").ok_or(error.clone())?;
            self.remainder = remainder;
            if token.to_uppercase().as_str() != "PTR" {
                return Err(error);
            }
        } else {
            // If there was no size directive, revert the remainder because it still needs to be
            // parsed.
            self.remainder = remainder_at_start;
        }

        Ok(size_directive)
    }

    fn parse_memory_reference(&mut self) -> Result<Option<Operand>, Error> {
        if self.remainder.len() < 3 {
            return Ok(None);
        }

        let mut chars = self.remainder.chars();
        if chars.nth(0).unwrap() != '[' {
            return Ok(None);
        }

        if chars.last().unwrap() != ']' {
            return Err(Error::CannotParseInstruction(
                "malformed memory reference (\"[\" not closed)".into(),
            ));
        }

        let inner = &self.remainder[1..self.remainder.len() - 2].to_lowercase();
        let mut operator = MemoryOperandOperator::Add;
        let mut memory_operand_sequence = MemoryOperandSequence::new();
        for token in inner.split_inclusive(&['+', '-', '*']) {
            if let Ok(register) = Register::try_from(token) {
                // Push will fail if we attempt to push three or more registers as this is invalid.
                memory_operand_sequence
                    .push(operator.clone(), MemoryOperandType::Register(register))?;
            }
            // Unsure if necessary or if u64 parsing has it built in.
            // if token.starts_with("0x") {
            // token = &token[2..];
            // }
            if let Ok(immediate) = token.parse::<u64>() {
                // Push can only fail when pushing registers, therefore we do not need to verify
                // the result.
                memory_operand_sequence
                    .push(operator.clone(), MemoryOperandType::Immediate(immediate));
            }
            if let Some(last) = token.chars().last() {
                if let Ok(new_operator) = MemoryOperandOperator::try_from(last) {
                    operator = new_operator;
                }
            }
        }

        todo!()
    }

    fn may_be_valid_immediate(value: &str) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    mod intel {
        use crate::error::Error;
        use crate::instruction::{OperandType, SizeDirective};

        use super::super::*;

        #[test]
        fn parse_immediate_operand() {
            assert_eq!(
                IntelOperandStrParser::parse("58").unwrap(),
                Operand::new(OperandType::Immediate(58), None)
            );
        }

        #[test]
        fn parse_memory_operand_no_size_directive() {
            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                IntelOperandStrParser::parse("[ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Ebx),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Eax),
                )
                .unwrap();

            assert_eq!(
                IntelOperandStrParser::parse("[ebx+eax]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Edx),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Subtract,
                    MemoryOperandType::Register(Register::Edi),
                )
                .unwrap();

            assert_eq!(
                IntelOperandStrParser::parse("[edx-edi]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Edx),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Immediate(4),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                IntelOperandStrParser::parse("[EAX+4*EBX]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Edx),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Immediate(4),
                )
                .unwrap();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Ebx),
                )
                .unwrap();

            assert!(
                IntelOperandStrParser::parse("[eax+EBX+ecx]").is_err()
            );
        }

        #[test]
        fn parse_memory_operand_with_size_directive() {
            let mut expected = MemoryOperandSequence::new();
            expected
                .push(
                    MemoryOperandOperator::Add,
                    MemoryOperandType::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                IntelOperandStrParser::parse("BYTE PTR [ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected.clone()), Some(SizeDirective::Byte))
            );

            assert_eq!(
                IntelOperandStrParser::parse("word ptr [ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected.clone()), Some(SizeDirective::Word))
            );

            assert_eq!(
                IntelOperandStrParser::parse("dword PTR [ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected), Some(SizeDirective::Dword))
            );
        }

        #[test]
        fn parse_register_operand() {}

        #[test]
        fn parse_invalid_operand() {}
    }
}
