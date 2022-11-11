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
        todo!()
        // self.remainder
        //     .split(",")
        //     .map(|o| NasmOperandStrParser::parse(o.trim()))
        //     .collect()
    }
}
