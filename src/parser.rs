use crate::{
    error::Error,
    instruction::{
        self, EffectiveAddress, EffectiveAddressOperand, EffectiveAddressOperator, Instruction,
        Operand, OperandType, Size,
    },
    register::Register,
};

pub struct NasmInstructionStrParser<'a> {
    remainder: &'a str,
}

impl<'a> NasmInstructionStrParser<'a> {
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
            .map(|o| NasmOperandStrParser::parse(o.trim()))
            .collect()
    }
}

// TODO: Rename to Parser and instead vary functionality based on taking `NasmStr`, or another
//       format. It also doesn't seem necessary to have an instruction parser and an operand
//       parser.
pub struct NasmOperandStrParser<'a> {
    remainder: &'a str,
}

impl<'a> NasmOperandStrParser<'a> {
    fn parse(operand: &'a str) -> Result<Operand, Error> {
        // Valid:
        //     BYTE PTR [ebx]
        //     BYTE [ebx]
        //     BYTE 2
        // TODO: BYTE 20000000 -> truncates or overflows, unsure which, but valid regardless
        //     2
        //     ebx
        //     [ebx]
        //     [ebx-1]
        //     [ebx+1]
        //     [ebx+eax]
        //     [ ebx + eax ]
        //     [ebx+4*eax]
        //     [ebx+eax*4]
        //     [eax*4+0x419260]
        //     Whitespace is entirely ignored. We can probably strip all whitespace within the
        //     operands.
        //     No limit to number of additions or multiplications of scalars.
        // Invalid:
        //     [ebx-eax] no subtraction with registers.
        //     [ebx*eax] no multiplication with registers.
        //     [ebx+eax+ecx] at most 2 registers.
        //     [eax+bx] must be of same size
        //     [ax]/[al] must be 32-bit on x86. Only 32-bit registers even in combination with
        //     others.
        // These are called "effective addresses" by nasm.
        // TODO: Eventually will need to parse variable names that refer to variables within the
        //       assembly file.
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

        // 2. Is it an effective address?
        if parser.remainder.contains('[') {
            let effective_address = parser.parse_effective_address()?;
            return Ok(Operand::new(
                OperandType::Memory(effective_address),
                size_directive,
            ));
        }

        // 3. Is it a register?
        // let operand = Operand::try_from(value)

        // 4. Is it an immediate value?

        todo!()
    }

    fn parse_size_directive(&mut self) -> Result<Option<Size>, Error> {
        let remainder_at_start = self.remainder;

        // FIXME: DWORD[EAX] is valid, no space is required.
        //        DWORD0 is not valid. Can check if '[' exists and split on that + trim, else split
        //        on ' '.
        let (token, remainder) = self
            .remainder
            .split_once(" ")
            .ok_or(Error::CannotParseInstruction("operand is empty".into()))?;
        self.remainder = remainder;

        let size_directive = match token.to_uppercase().as_str() {
            "BYTE" => Some(Size::Byte),
            "WORD" => Some(Size::Word),
            "DWORD" => Some(Size::Dword),
            "QWORD" => Some(Size::Qword),
            _ => None,
        };

        if size_directive.is_some() {
            if let Some((token, _)) = remainder.split_once(" ") {
                if token.to_uppercase().as_str() == "PTR" {
                    return Err(Error::CannotParseInstruction(
                        "NASM syntax does not use the \"PTR\" keyword".into(),
                    ));
                }
            }
        } else {
            // No size directive: revert remainder so that it may be parsed again.
            self.remainder = remainder_at_start;
        }

        Ok(size_directive)
    }

    fn parse_effective_address(&mut self) -> Result<EffectiveAddress, Error> {
        let mut chars = self.remainder.chars();
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

        if self.remainder.len() < 3 {
            return Err(Error::CannotParseInstruction(
                "invalid effective address (no contents)".into(),
            ));
        }

        let inner = &self.remainder[1..self.remainder.len() - 2].to_lowercase();
        let mut operator = EffectiveAddressOperator::Add;
        let mut memory_operand_sequence = EffectiveAddress::new();
        for mut token in inner.split_inclusive(&['+', '-', '*']) {
            let next_operator = if let Ok(next_operator) =
                EffectiveAddressOperator::try_from(token.chars().last().unwrap())
            {
                // Remove the trailing operand and trim since whitespace is irrelevant.
                token = token[0..token.len() - 2].trim();
                next_operator
            } else {
                // Irrelevant: this is the final iteration.
                EffectiveAddressOperator::Add
            };
            // Only 32-bit registers are valid.
            // let operand = EffectiveAddressOperand::try_from(token)?;
            // if let EffectiveAddressOperand::Register(register) = &operand {
            //     if !(register == &Register::Eax
            //         || register == &Register::Ebx
            //         || register == &Register::Ecx
            //         || register == &Register::Edx
            //         || register == &Register::Edi
            //         || register == &Register::Esi
            //         || register == &Register::Ebp
            //         || register == &Register::Esp)
            //     {
            //         return Err(Error::CannotParseInstruction(
            //             "invalid effective address (must use 32-bit registers)".into(),
            //         ));
            //     }
            // }
            // memory_operand_sequence.push(operator, operand)?;
            // operator = next_operator;
        }

        Ok(memory_operand_sequence)
    }

    fn may_be_valid_immediate(value: &str) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    mod nasm {
        use crate::error::Error;
        use crate::instruction::{Immediate, Operand, OperandType, Size};

        use super::super::*;

        #[test]
        fn parse_immediate_operand() {
            assert_eq!(
                NasmOperandStrParser::parse("58").unwrap(),
                Operand::new(
                    OperandType::Immediate(Immediate::new("58".into(), 58)),
                    None
                )
            );
        }

        #[test]
        fn parse_effective_address_operand_without_size_directive() {
            let mut expected = EffectiveAddress::new();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                NasmOperandStrParser::parse("[ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = EffectiveAddress::new();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Register(Register::Ebx),
                )
                .unwrap();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Register(Register::Eax),
                )
                .unwrap();

            assert_eq!(
                NasmOperandStrParser::parse("[ ebx +  eax ]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );

            let mut expected = EffectiveAddress::new();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Register(Register::Edx),
                )
                .unwrap();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Immediate(4),
                )
                .unwrap();
            expected
                .push(
                    EffectiveAddressOperator::Multiply,
                    EffectiveAddressOperand::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                NasmOperandStrParser::parse("[EAX+4*EBX]").unwrap(),
                Operand::new(OperandType::Memory(expected), None)
            );
        }

        #[test]
        fn parse_effective_address_with_size_directive() {
            let mut expected = EffectiveAddress::new();
            expected
                .push(
                    EffectiveAddressOperator::Add,
                    EffectiveAddressOperand::Register(Register::Ebx),
                )
                .unwrap();

            assert_eq!(
                NasmOperandStrParser::parse("BYTE [ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected.clone()), Some(Size::Byte))
            );

            assert_eq!(
                NasmOperandStrParser::parse("word        [ebx]").unwrap(),
                Operand::new(OperandType::Memory(expected.clone()), Some(Size::Word))
            );

            assert_eq!(
                NasmOperandStrParser::parse("dword[ ebx    ]").unwrap(),
                Operand::new(OperandType::Memory(expected.clone()), Some(Size::Dword))
            );

            assert_eq!(
                NasmOperandStrParser::parse("QwOrD    [    ebx   ]").unwrap(),
                Operand::new(OperandType::Memory(expected), Some(Size::Qword))
            );
        }

        #[test]
        fn parse_invalid_effective_address() {
            assert!(NasmOperandStrParser::parse("[eax+EBX+ecx]").is_err());
            assert!(NasmOperandStrParser::parse("[edx-edi]").is_err());
            assert!(NasmOperandStrParser::parse("[edx+ax]").is_err());
            assert!(NasmOperandStrParser::parse("[ax+al]").is_err());
            assert!(NasmOperandStrParser::parse("[cs+dx]").is_err());
            assert!(NasmOperandStrParser::parse("[ax]").is_err());
            assert!(NasmOperandStrParser::parse("[al]").is_err());
            assert!(NasmOperandStrParser::parse("[eax*10]").is_err());
            assert!(NasmOperandStrParser::parse("a[eax]").is_err());
            assert!(NasmOperandStrParser::parse("eax]").is_err());
            assert!(NasmOperandStrParser::parse("[eax").is_err());
            assert!(NasmOperandStrParser::parse("20+[eax]").is_err());
            assert!(NasmOperandStrParser::parse("[eip]").is_err());
            assert!(NasmOperandStrParser::parse("[eflags]").is_err());
            assert!(NasmOperandStrParser::parse("DWORD PTR [eax]").is_err());
            assert!(NasmOperandStrParser::parse("WORDPTR[eax]").is_err());
            assert!(NasmOperandStrParser::parse("[eax]a").is_err());
            assert!(NasmOperandStrParser::parse("a[eax]").is_err());
        }

        #[test]
        fn parse_register_operand() {
            // assert_eq!(NasmOperandStrParser::parse("eax").unwrap(), Operand::)
        }

        #[test]
        fn parse_invalid_operand() {}
    }
}
