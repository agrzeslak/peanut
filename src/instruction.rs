use bitflags::bitflags;

use crate::{error::Error, register::Register, cpu::Cpu};

bitflags! {
    struct Operands: u32 {
        const IMM8 = 0x1;
        const IMM16 = 0x2;
        const IMM32 = 0x4;
        const IMM = Self::IMM8.bits | Self::IMM16.bits | Self::IMM32.bits;

        const M8 = 0x8;
        const M16 = 0x10;
        const M32 = 0x20;
        const M = Self::M8.bits | Self::M16.bits | Self::M32.bits;

        const R8 = 0x40;
        const R16 = 0x80;
        const R32 = 0x100;
        const R = Self::R8.bits | Self::R16.bits | Self::R32.bits;

        const EAX = 0x200;
        const AX = 0x400;
        const AH = 0x800;
        const AL = 0x1000;

        const EBX = 0x2000;
        const BX = 0x4000;
        const BH = 0x8000;
        const BL = 0x10000;

        const ECX = 0x20000;
        const CX = 0x40000;
        const CH = 0x80000;
        const CL = 0x100000;

        const EDX = 0x200000;
        const DX = 0x400000;
        const DH = 0x800000;
        const DL = 0x1000000;

        const EDI = 0x2000000;
        const DI = 0x4000000;

        const ESI = 0x8000000;
        const SI = 0x10000000;

        const DS = 0x20000000;
        const SS = 0x40000000;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InstructionSignature<'a> {
    op_code: u32,
    mnemonic: &'a str,
    operand1: Operands,
    operand2: Operands,
    operand3: Operands,
    operand4: Operands,
    // TODO: According to
    // https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/x86-instructions, "Unless
    // otherwise noted, ... you cannot choose memory for both source and destination". Look into
    // this further.
    source_and_destination_can_be_memory: bool,
    // function: Fn
}

// TT muncher. There is likely a much more elegant way to accomplish this.
macro_rules! expand_operands {
    () => { Operands::empty() };

    (imm8) => { Operands::IMM8 };
    (imm16) => { Operands::IMM16 };
    (imm32) => { Operands::IMM32 };
    (imm) => { Operands::IMM };

    (m8) => { Operands::M8 };
    (m16) => { Operands::M16 };
    (m32) => { Operands::M32 };
    (m) => { Operands::M };

    (r8) => { Operands::R8 };
    (r16) => { Operands::R16 };
    (r32) => { Operands::R32 };
    (r) => { Operands::R };

    (eax) => { Operands::EAX };
    (ax) => { Operands::AX };
    (ah) => { Operands::AH };
    (al) => { Operands::AL };

    (ebx) => { Operands::EBX };
    (bx) => { Operands::BX };
    (bh) => { Operands::BH };
    (bl) => { Operands::BL };

    (ecx) => { Operands::ECX };
    (cx) => { Operands::CX };
    (ch) => { Operands::CH };
    (cl) => { Operands::CL };

    (edx) => { Operands::EDX };
    (dx) => { Operands::DX };
    (dh) => { Operands::DH };
    (dl) => { Operands::DL };

    (edi) => { Operands::EDI };
    (di) => { Operands::DI };

    (esi) => { Operands::ESI };
    (si) => { Operands::SI };

    (ds) => { Operands::DS };
    (ss) => { Operands::SS };

    ($operand:ident) => {
        expand_operands!($operand)
    };
    ($first_operand:ident, $second_operand:ident) => {
        expand_operands!($first_operand).union(expand_operands!($second_operand))
    };
    ($first_operand:ident, $($other_operands:tt)*) => {
        expand_operands!($($other_operands)*)
    };
}

macro_rules! construct_signature {
    ($op_code:literal, $mnemonic:literal, [$($operand1:tt)*], [$($operand2:tt)*], [$($operand3:tt)*], [$($operand4:tt)*], $source_and_destination_can_be_memory:literal) => {
        InstructionSignature {
            op_code: $op_code,
            mnemonic: $mnemonic,
            operand1: expand_operands!($($operand1)*),
            operand2: expand_operands!($($operand2)*),
            operand3: expand_operands!($($operand3)*),
            operand4: expand_operands!($($operand4)*),
            source_and_destination_can_be_memory: $source_and_destination_can_be_memory,
        }
    }
}

// FIXME: https://github.com/bitflags/bitflags/issues/180 cannot currently use `|` operator in
//        const contexts for creating unions of these flags. `union` method is the current
//        workaround.
const INSTRUCTIONS: [InstructionSignature; 225] = [
    construct_signature!(0x00, "ADD", [r, m8], [r8], [], [], false),
    construct_signature!(0x01, "ADD", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x02, "ADD", [r8], [m8], [], [], false),
    construct_signature!(0x03, "ADD", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x04, "ADD", [imm8], [], [], [], false),
    construct_signature!(0x05, "ADD", [imm16, imm32], [], [], [], false),
    construct_signature!(0x06, "PUSH", [], [], [], [], false),
    construct_signature!(0x07, "POP", [], [], [], [], false),
    construct_signature!(0x08, "OR", [r, m8], [r8], [], [], false),
    construct_signature!(0x09, "OR", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x0A, "OR", [r8], [r, m8], [], [], false),
    construct_signature!(0x0B, "OR", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x0C, "OR", [imm8], [], [], [], false),
    construct_signature!(0x0D, "OR", [imm16, imm32], [], [], [], false),
    construct_signature!(0x0E, "PUSH", [], [], [], [], false),
    construct_signature!(0x10, "ADC", [r, m8], [r8], [], [], false),
    construct_signature!(0x11, "ADC", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x12, "ADC", [r8], [r, m8], [], [], false),
    construct_signature!(0x13, "ADC", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x14, "ADC", [al], [imm8], [], [], false),
    construct_signature!(0x15, "ADC", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x16, "PUSH", [ss], [], [], [], false),
    construct_signature!(0x17, "POP", [ss], [], [], [], false),
    construct_signature!(0x18, "SBB", [r, m8], [r8], [], [], false),
    construct_signature!(0x19, "SBB", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x1A, "SBB", [r8], [r, m8], [], [], false),
    construct_signature!(0x1B, "SBB", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x1C, "SBB", [al], [imm8], [], [], false),
    construct_signature!(0x1D, "SBB", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x1E, "PUSH", [ds], [], [], [], false),
    construct_signature!(0x1F, "POP", [ds], [], [], [], false),
    construct_signature!(0x20, "AND", [r, m8], [r8], [], [], false),
    construct_signature!(0x21, "AND", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x22, "AND", [r8], [r, m8], [], [], false),
    construct_signature!(0x23, "AND", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x24, "AND", [al], [imm8], [], [], false),
    construct_signature!(0x25, "AND", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x26, "ES", [], [], [], [], false),
    construct_signature!(0x27, "DAA", [], [], [], [], false),
    construct_signature!(0x28, "SUB", [r, m8], [r8], [], [], false),
    construct_signature!(0x29, "SUB", [r, m16, m32], [r16, r32 ], [], [], false),
    construct_signature!(0x2A, "SUB", [r8], [r, m8], [], [], false),
    construct_signature!(0x2B, "SUB", [r16, r32], [r, m16, m32 ], [], [], false),
    construct_signature!(0x2C, "SUB", [al], [imm8], [], [], false),
    construct_signature!(0x2D, "SUB", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x2E, "CS", [], [], [], [], false),
    construct_signature!(0x2F, "DAS", [], [], [], [], false),
    construct_signature!(0x30, "XOR", [r, m8], [r8 ], [], [], false),
    construct_signature!(0x31, "XOR", [r, m16, m32], [r16, r32 ], [], [], false),
    construct_signature!(0x32, "XOR", [r8], [r, m8], [], [], false),
    construct_signature!(0x33, "XOR", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x34, "XOR", [al], [imm8], [], [], false),
    construct_signature!(0x35, "XOR", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x36, "SS", [], [], [], [], false),
    construct_signature!(0x37, "AAA", [], [], [], [], false),
    construct_signature!(0x38, "CMP", [r, m8], [r8], [], [], false),
    construct_signature!(0x39, "CMP", [r, m16, m32], [r16, r32], [], [], false),
    construct_signature!(0x3A, "CMP", [r8], [r, m8], [], [], false),
    construct_signature!(0x3B, "CMP", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x3C, "CMP", [al], [imm8], [], [], false),
    construct_signature!(0x3D, "CMP", [eax], [imm16, imm32], [], [], false),
    construct_signature!(0x3E, "DS", [], [], [], [], false),
    construct_signature!(0x3F, "AAS", [], [], [], [], false),
    construct_signature!(0x40, "INC", [r16, r32], [], [], [], false),
    construct_signature!(0x48, "DEC", [r16, r32], [], [], [], false),
    construct_signature!(0x50, "PUSH", [r16, r32], [], [], [], false),
    construct_signature!(0x58, "POP", [r16, r32], [], [], [], false),
    construct_signature!(0x60, "PUSHA", [], [], [], [], false),
    construct_signature!(0x60, "PUSHAD", [], [], [], [], false),
    construct_signature!(0x61, "POPA", [], [], [], [], false),
    construct_signature!(0x61, "POPAD", [], [], [], [], false),
    // construct_signature!(
    //     0x62,
    //     "BOUND",
    //     [r16, r32],
    //     [m16, m32 & 16 / 32],
    //     [],
    //     [],
    //     false
    // ),
    construct_signature!(0x63, "ARPL", [r, m16], [r16], [], [], false),
    construct_signature!(0x64, "FS", [], [], [], [], false),
    construct_signature!(0x65, "GS", [], [], [], [], false),
    // 0x66 no mnemonic
    // 0x66 no mnemonic sse2
    // 0x67 no mnemonic
    construct_signature!(0x68, "PUSH", [imm16, imm32], [], [], [], false),
    construct_signature!(0x69, "IMUL", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x6A, "PUSH", [imm8], [], [], [], false),
    construct_signature!(0x6B, "IMUL", [r16, r32], [r, m16, m32], [], [], false),
    construct_signature!(0x6C, "", [], [], [], [], false),
    construct_signature!(0x6D, "", [], [], [], [], false),
    construct_signature!(0x6E, "", [], [], [], [], false),
    construct_signature!(0x6F, "", [], [], [], [], false),
    construct_signature!(0x70, "", [], [], [], [], false),
    construct_signature!(0x71, "", [], [], [], [], false),
    construct_signature!(0x72, "", [], [], [], [], false),
    construct_signature!(0x73, "", [], [], [], [], false),
    construct_signature!(0x74, "", [], [], [], [], false),
    construct_signature!(0x75, "", [], [], [], [], false),
    construct_signature!(0x76, "", [], [], [], [], false),
    construct_signature!(0x77, "", [], [], [], [], false),
    construct_signature!(0x78, "", [], [], [], [], false),
    construct_signature!(0x79, "", [], [], [], [], false),
    construct_signature!(0x7A, "", [], [], [], [], false),
    construct_signature!(0x7B, "", [], [], [], [], false),
    construct_signature!(0x7C, "", [], [], [], [], false),
    construct_signature!(0x7D, "", [], [], [], [], false),
    construct_signature!(0x7E, "", [], [], [], [], false),
    construct_signature!(0x7F, "", [], [], [], [], false),
    construct_signature!(0x80, "", [], [], [], [], false),
    construct_signature!(0x81, "", [], [], [], [], false),
    construct_signature!(0x82, "", [], [], [], [], false),
    construct_signature!(0x83, "", [], [], [], [], false),
    construct_signature!(0x84, "", [], [], [], [], false),
    construct_signature!(0x85, "", [], [], [], [], false),
    construct_signature!(0x86, "", [], [], [], [], false),
    construct_signature!(0x87, "", [], [], [], [], false),
    construct_signature!(0x88, "MOV", [r, m8], [r8], [], [], false),
    construct_signature!(0x89, "", [], [], [], [], false),
    construct_signature!(0x8A, "", [], [], [], [], false),
    construct_signature!(0x8B, "", [], [], [], [], false),
    construct_signature!(0x8C, "", [], [], [], [], false),
    construct_signature!(0x8D, "LEA", [r], [m], [], [], false),
    construct_signature!(0x8E, "", [], [], [], [], false),
    construct_signature!(0x8F, "", [], [], [], [], false),
    construct_signature!(0x90, "", [], [], [], [], false),
    construct_signature!(0x91, "", [], [], [], [], false),
    construct_signature!(0x92, "", [], [], [], [], false),
    construct_signature!(0x93, "", [], [], [], [], false),
    construct_signature!(0x94, "", [], [], [], [], false),
    construct_signature!(0x95, "", [], [], [], [], false),
    construct_signature!(0x96, "", [], [], [], [], false),
    construct_signature!(0x97, "", [], [], [], [], false),
    construct_signature!(0x98, "", [], [], [], [], false),
    construct_signature!(0x99, "", [], [], [], [], false),
    construct_signature!(0x9A, "", [], [], [], [], false),
    construct_signature!(0x9B, "", [], [], [], [], false),
    construct_signature!(0x9C, "", [], [], [], [], false),
    construct_signature!(0x9D, "", [], [], [], [], false),
    construct_signature!(0x9E, "", [], [], [], [], false),
    construct_signature!(0x9F, "", [], [], [], [], false),
    construct_signature!(0xA0, "", [], [], [], [], false),
    construct_signature!(0xA1, "", [], [], [], [], false),
    construct_signature!(0xA2, "", [], [], [], [], false),
    construct_signature!(0xA3, "", [], [], [], [], false),
    construct_signature!(0xA4, "", [], [], [], [], false),
    construct_signature!(0xA5, "", [], [], [], [], false),
    construct_signature!(0xA6, "", [], [], [], [], false),
    construct_signature!(0xA7, "", [], [], [], [], false),
    construct_signature!(0xA8, "", [], [], [], [], false),
    construct_signature!(0xA9, "", [], [], [], [], false),
    construct_signature!(0xAA, "", [], [], [], [], false),
    construct_signature!(0xAB, "", [], [], [], [], false),
    construct_signature!(0xAC, "", [], [], [], [], false),
    construct_signature!(0xAD, "", [], [], [], [], false),
    construct_signature!(0xAE, "", [], [], [], [], false),
    construct_signature!(0xAF, "", [], [], [], [], false),
    construct_signature!(0xB0, "", [], [], [], [], false),
    construct_signature!(0xB1, "", [], [], [], [], false),
    construct_signature!(0xB2, "", [], [], [], [], false),
    construct_signature!(0xB3, "", [], [], [], [], false),
    construct_signature!(0xB4, "", [], [], [], [], false),
    construct_signature!(0xB5, "", [], [], [], [], false),
    construct_signature!(0xB6, "", [], [], [], [], false),
    construct_signature!(0xB7, "", [], [], [], [], false),
    construct_signature!(0xB8, "", [], [], [], [], false),
    construct_signature!(0xB9, "", [], [], [], [], false),
    construct_signature!(0xBA, "", [], [], [], [], false),
    construct_signature!(0xBB, "", [], [], [], [], false),
    construct_signature!(0xBC, "", [], [], [], [], false),
    construct_signature!(0xBD, "", [], [], [], [], false),
    construct_signature!(0xBE, "", [], [], [], [], false),
    construct_signature!(0xBF, "", [], [], [], [], false),
    construct_signature!(0xC0, "", [], [], [], [], false),
    construct_signature!(0xC1, "", [], [], [], [], false),
    construct_signature!(0xC2, "", [], [], [], [], false),
    construct_signature!(0xC3, "", [], [], [], [], false),
    construct_signature!(0xC4, "", [], [], [], [], false),
    construct_signature!(0xC5, "", [], [], [], [], false),
    construct_signature!(0xC6, "", [], [], [], [], false),
    construct_signature!(0xC7, "", [], [], [], [], false),
    construct_signature!(0xC8, "", [], [], [], [], false),
    construct_signature!(0xC9, "", [], [], [], [], false),
    construct_signature!(0xCA, "", [], [], [], [], false),
    construct_signature!(0xCB, "", [], [], [], [], false),
    construct_signature!(0xCC, "", [], [], [], [], false),
    construct_signature!(0xCD, "", [], [], [], [], false),
    construct_signature!(0xCE, "", [], [], [], [], false),
    construct_signature!(0xCF, "", [], [], [], [], false),
    construct_signature!(0xD0, "", [], [], [], [], false),
    construct_signature!(0xD1, "", [], [], [], [], false),
    construct_signature!(0xD2, "", [], [], [], [], false),
    construct_signature!(0xD3, "", [], [], [], [], false),
    construct_signature!(0xD4, "", [], [], [], [], false),
    construct_signature!(0xD5, "", [], [], [], [], false),
    construct_signature!(0xD6, "", [], [], [], [], false),
    construct_signature!(0xD7, "", [], [], [], [], false),
    construct_signature!(0xD8, "", [], [], [], [], false),
    construct_signature!(0xD9, "", [], [], [], [], false),
    construct_signature!(0xDA, "", [], [], [], [], false),
    construct_signature!(0xDB, "", [], [], [], [], false),
    construct_signature!(0xDC, "", [], [], [], [], false),
    construct_signature!(0xDD, "", [], [], [], [], false),
    construct_signature!(0xDE, "", [], [], [], [], false),
    construct_signature!(0xDF, "", [], [], [], [], false),
    construct_signature!(0xE0, "", [], [], [], [], false),
    construct_signature!(0xE1, "", [], [], [], [], false),
    construct_signature!(0xE2, "", [], [], [], [], false),
    construct_signature!(0xE3, "", [], [], [], [], false),
    construct_signature!(0xE4, "", [], [], [], [], false),
    construct_signature!(0xE5, "", [], [], [], [], false),
    construct_signature!(0xE6, "", [], [], [], [], false),
    construct_signature!(0xE7, "", [], [], [], [], false),
    construct_signature!(0xE8, "", [], [], [], [], false),
    construct_signature!(0xE9, "", [], [], [], [], false),
    construct_signature!(0xEA, "", [], [], [], [], false),
    construct_signature!(0xEB, "", [], [], [], [], false),
    construct_signature!(0xEC, "", [], [], [], [], false),
    construct_signature!(0xED, "", [], [], [], [], false),
    construct_signature!(0xEE, "", [], [], [], [], false),
    construct_signature!(0xEF, "", [], [], [], [], false),
    construct_signature!(0xF0, "", [], [], [], [], false),
    construct_signature!(0xF1, "", [], [], [], [], false),
    construct_signature!(0xF2, "", [], [], [], [], false),
    construct_signature!(0xF3, "", [], [], [], [], false),
    construct_signature!(0xF4, "", [], [], [], [], false),
    construct_signature!(0xF5, "", [], [], [], [], false),
    construct_signature!(0xF6, "", [], [], [], [], false),
    construct_signature!(0xF7, "", [], [], [], [], false),
    construct_signature!(0xF8, "", [], [], [], [], false),
    construct_signature!(0xF9, "", [], [], [], [], false),
    construct_signature!(0xFA, "", [], [], [], [], false),
    construct_signature!(0xFB, "", [], [], [], [], false),
    construct_signature!(0xFC, "", [], [], [], [], false),
    construct_signature!(0xFD, "", [], [], [], [], false),
    construct_signature!(0xFE, "", [], [], [], [], false),
];

pub enum Operand {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    M8(u8),
    M16(u16),
    M32(u32),
    R8(Register),
    R16(Register),
    R32(Register),
}

// https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/x86-instructions
// https://www.felixcloutier.com/x86/index.html
// http://ref.x86asm.net/coder32.html
// TODO: Perhaps strictly encode into the types what exact operands each instruction can take. For
//       instance if you can only target a register, only accept a register.
// TODO: Should we not have these as an enum, but rather just have an instruction trait? This would
//       mean that all the instructions wouldn't have to take up the same amount of space
//       regardless of how many operands they require.
pub enum Instruction {
    // Effective address
    Lea(Operand, Operand),

    // Data transfer
    Mov(Operand, Operand),
    Movsx(Operand, Operand),

    // Stack manipulation
    Push(Operand),
    Pop(Operand),
    Pushfd(),
    Popfd(),
    Pushad(),
    Popad(),
    Enter(Operand, Operand),
    Leave(),

    // Data conversion
    Cbw(),
    Cwd(),
    Cwde(),
    Cdq(),

    // Arithmetic
    Add(Operand, Operand),
    Adc(Operand, Operand),
    Sub(Operand, Operand),
    Sbb(Operand, Operand),
    Neg(Operand),
    Inc(Operand),
    Dec(Operand),
    Cmp(Operand, Operand),
    // TODO: Mul/Imul have multiple variabnts based on the size of the memory provided?
    Mul(Operand),
    Imul(Operand, Option<Operand>),
    // TODO: Div/Idiv have multiple variabnts based on the size of the memory provided?
    Div(Operand),
    Idiv(Operand),
    // SETcc: https://www.felixcloutier.com/x86/setcc.html
    Seta(Operand),
    Setae(Operand),
    Setb(Operand),
    Setbe(Operand),
    Setc(Operand),
    Sete(Operand),
    Setg(Operand),
    Setge(Operand),
    Setl(Operand),
    Setle(Operand),
    Setna(Operand),
    Setnae(Operand),
    Setnb(Operand),
    Setnbe(Operand),
    Setnc(Operand),
    Setne(Operand),
    Setng(Operand),
    Setnge(Operand),
    Setnl(Operand),
    Setnle(Operand),
    Setno(Operand),
    Setnp(Operand),
    Setns(Operand),
    Setnz(Operand),
    Seto(Operand),
    Setp(Operand),
    Setpe(Operand),
    Setpo(Operand),
    Sets(Operand),
    Setz(Operand),

    // Binary-coded decimal
    Daa(),
    Das(),
    Aaa(),
    Aas(),
    Aam(),
    Aad(),

    // Bits
    And(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),
    Not(Operand),
    Test(Operand, Operand),
    Shl(Operand, Operand),
    Shr(Operand, Operand),
    Sar(Operand, Operand),
    Shld(Operand, Operand),
    Shrd(Operand, Operand),
    Rol(Operand, Operand),
    Ror(Operand, Operand),
    Rcl(Operand, Operand),
    Rcr(Operand, Operand),
    Bt(Operand, Operand),
    Bts(Operand, Operand),
    Btc(Operand, Operand),

    // Control flow
    // Jcc: https://www.felixcloutier.com/x86/jcc.html
    // NOTE: Different opcodes depending on operand sizes.
    Ja(Operand),
    Jae(Operand),
    Jb(Operand),
    Jbe(Operand),
    Jc(Operand),
    Jcxz(Operand),
    Jecxz(Operand),
    Je(Operand),
    Jg(Operand),
    Jge(Operand),
    Jl(Operand),
    Jle(Operand),
    Jna(Operand),
    Jnae(Operand),
    Jnb(Operand),
    Jnbe(Operand),
    Jnc(Operand),
    Jne(Operand),
    Jng(Operand),
    Jnge(Operand),
    Jnl(Operand),
    Jnle(Operand),
    Jno(Operand),
    Jnp(Operand),
    Jns(Operand),
    Jnz(Operand),
    Jo(Operand),
    Jp(Operand),
    Jpe(Operand),
    Jpo(Operand),
    Js(Operand),
    Jz(Operand),
    Jmp(Operand),
    Call(Operand),
    Ret(Operand),
    Loop(),
    Loopz(),
    Loopnz(),

    // String manipulation (legacy, slower than equivalent instructions written out the long way)
    // TODO: Fill in.

    // Flags
    Lahf(),
    Sahf(),
    Stc(),
    Clc(),
    Cmc(),
    Std(),
    Cld(),
    Sti(),
    Cli(),

    // Interlocked instructions
    Xchg(Operand, Operand),
    Xadd(Operand, Operand),
    Cmpxchg(Operand, Operand),

    // Miscellanous
    Int(Operand),
    Bound(Operand, Operand),
    Nop(),
    Xlatb(),
    Bswap(Operand),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_operands() {
        assert_eq!(expand_operands!(), Operands::empty());
        assert_eq!(expand_operands!(imm8), Operands::IMM8);
        assert_eq!(
            expand_operands!(imm8, imm16),
            Operands::IMM8.union(Operands::IMM16)
        );
        assert_eq!(
            expand_operands!(imm),
            Operands::IMM8.union(Operands::IMM16).union(Operands::IMM32)
        );
        assert_eq!(
            expand_operands!(m32, m, r32),
            Operands::M32.union(Operands::M).union(Operands::R32)
        );
    }

    #[test]
    fn construct_signature() {
        let manual_instruction_signature = InstructionSignature {
            op_code: 0x8D,
            mnemonic: "LEA",
            operand1: Operands::IMM8.union(Operands::M8),
            operand2: Operands::IMM8.union(Operands::R8),
            operand3: Operands::IMM8.union(Operands::IMM16),
            operand4: Operands::empty(),
            source_and_destination_can_be_memory: false,
        };
        let macro_instruction_signature = construct_signature!(
            0x8D,
            "LEA",
            [imm8, m8],
            [imm8, r8],
            [imm8, imm16],
            [],
            false
        );
        assert_eq!(macro_instruction_signature, manual_instruction_signature);
        assert_eq!(macro_instruction_signature.mnemonic, "LEA");
        assert_eq!(
            macro_instruction_signature.operand1,
            Operands::IMM8.union(Operands::M8)
        );
        assert_eq!(
            macro_instruction_signature.operand2,
            Operands::IMM8.union(Operands::R8)
        );
        assert_eq!(
            macro_instruction_signature.operand3,
            Operands::IMM8.union(Operands::IMM16)
        );
        assert_eq!(macro_instruction_signature.operand4, Operands::empty());
        assert_eq!(
            macro_instruction_signature.source_and_destination_can_be_memory,
            false
        );
    }
}
