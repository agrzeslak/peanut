use crate::{memory::Address};

pub enum Operand<'a> {
    Immediate(Address),
    // Register(&'a Register),
    // To make compile
    Register(&'a Address),
}

// https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/x86-instructions
// https://www.felixcloutier.com/x86/index.html
// http://ref.x86asm.net/coder32.html
// TODO: Perhaps strictly encode into the types what exact operands each instruction can take. For
//       instance if you can only target a register, only accept a register.
// TODO: Should we not have these as an enum, but rather just have an instruction trait? This would
//       mean that all the instructions wouldn't have to take up the same amount of space
//       regardless of how many operands they require.
pub enum Instruction<'a, 'b> {
    // Effective address
    Lea(Operand<'a>, Operand<'b>),

    // Data transfer
    Mov(Operand<'a>, Operand<'b>),
    Movsx(Operand<'a>, Operand<'b>),

    // Stack manipulation
    Push(Operand<'a>),
    Pop(Operand<'a>),
    Pushfd(),
    Popfd(),
    Pushad(),
    Popad(),
    Enter(Operand<'a>, Operand<'b>),
    Leave(),

    // Data conversion
    Cbw(),
    Cwd(),
    Cwde(),
    Cdq(),

    // Arithmetic
    Add(Operand<'a>, Operand<'b>),
    Adc(Operand<'a>, Operand<'b>),
    Sub(Operand<'a>, Operand<'b>),
    Sbb(Operand<'a>, Operand<'b>),
    Neg(Operand<'a>),
    Inc(Operand<'a>),
    Dec(Operand<'a>),
    Cmp(Operand<'a>, Operand<'b>),
    // TODO: Mul/Imul have multiple variabnts based on the size of the memory provided?
    Mul(Operand<'a>),
    Imul(Operand<'a>, Option<Operand<'b>>),
    // TODO: Div/Idiv have multiple variabnts based on the size of the memory provided?
    Div(Operand<'a>),
    Idiv(Operand<'a>),
    // SETcc: https://www.felixcloutier.com/x86/setcc.html
    Seta(Operand<'a>),
    Setae(Operand<'a>),
    Setb(Operand<'a>),
    Setbe(Operand<'a>),
    Setc(Operand<'a>),
    Sete(Operand<'a>),
    Setg(Operand<'a>),
    Setge(Operand<'a>),
    Setl(Operand<'a>),
    Setle(Operand<'a>),
    Setna(Operand<'a>),
    Setnae(Operand<'a>),
    Setnb(Operand<'a>),
    Setnbe(Operand<'a>),
    Setnc(Operand<'a>),
    Setne(Operand<'a>),
    Setng(Operand<'a>),
    Setnge(Operand<'a>),
    Setnl(Operand<'a>),
    Setnle(Operand<'a>),
    Setno(Operand<'a>),
    Setnp(Operand<'a>),
    Setns(Operand<'a>),
    Setnz(Operand<'a>),
    Seto(Operand<'a>),
    Setp(Operand<'a>),
    Setpe(Operand<'a>),
    Setpo(Operand<'a>),
    Sets(Operand<'a>),
    Setz(Operand<'a>),

    // Binary-coded decimal
    Daa(),
    Das(),
    Aaa(),
    Aas(),
    Aam(),
    Aad(),

    // Bits
    And(Operand<'a>, Operand<'b>),
    Or(Operand<'a>, Operand<'b>),
    Xor(Operand<'a>, Operand<'b>),
    Not(Operand<'a>),
    Test(Operand<'a>, Operand<'b>),
    Shl(Operand<'a>, Operand<'b>),
    Shr(Operand<'a>, Operand<'b>),
    Sar(Operand<'a>, Operand<'b>),
    Shld(Operand<'a>, Operand<'b>),
    Shrd(Operand<'a>, Operand<'b>),
    Rol(Operand<'a>, Operand<'b>),
    Ror(Operand<'a>, Operand<'b>),
    Rcl(Operand<'a>, Operand<'b>),
    Rcr(Operand<'a>, Operand<'b>),
    Bt(Operand<'a>, Operand<'b>),
    Bts(Operand<'a>, Operand<'b>),
    Btc(Operand<'a>, Operand<'b>),

    // Control flow
    // Jcc: https://www.felixcloutier.com/x86/jcc.html
    // NOTE: Different opcodes depending on operand sizes.
    Ja(Operand<'a>),
    Jae(Operand<'a>),
    Jb(Operand<'a>),
    Jbe(Operand<'a>),
    Jc(Operand<'a>),
    Jcxz(Operand<'a>),
    Jecxz(Operand<'a>),
    Je(Operand<'a>),
    Jg(Operand<'a>),
    Jge(Operand<'a>),
    Jl(Operand<'a>),
    Jle(Operand<'a>),
    Jna(Operand<'a>),
    Jnae(Operand<'a>),
    Jnb(Operand<'a>),
    Jnbe(Operand<'a>),
    Jnc(Operand<'a>),
    Jne(Operand<'a>),
    Jng(Operand<'a>),
    Jnge(Operand<'a>),
    Jnl(Operand<'a>),
    Jnle(Operand<'a>),
    Jno(Operand<'a>),
    Jnp(Operand<'a>),
    Jns(Operand<'a>),
    Jnz(Operand<'a>),
    Jo(Operand<'a>),
    Jp(Operand<'a>),
    Jpe(Operand<'a>),
    Jpo(Operand<'a>),
    Js(Operand<'a>),
    Jz(Operand<'a>),
    Jmp(Operand<'a>),
    Call(Operand<'a>),
    Ret(Operand<'a>),
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
    Xchg(Operand<'a>, Operand<'b>),
    Xadd(Operand<'a>, Operand<'b>),
    Cmpxchg(Operand<'a>, Operand<'b>),

    // Miscellanous
    Int(Operand<'a>),
    Bound(Operand<'a>, Operand<'b>),
    Nop(),
    Xlatb(),
    Bswap(Operand<'a>),
}

pub struct Address8(u8);
pub struct Address16(u16);
pub struct Address32(u32);

pub enum OperandType {
    Imm8,
    Imm16,
    Imm32,
    M8,
    M16,
    M32,
    R8,
    R16,
    R32,
}

