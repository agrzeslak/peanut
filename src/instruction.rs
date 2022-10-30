use crate::{register::Register, memory::Address};

pub enum Operand<'a> {
    Immediate(Address),
    Register(&'a Register),
}

pub enum Instruction<'a, 'b> {
    Lea(Operand<'a>, Operand<'b>),
    Mov(Operand<'a>, Operand<'b>),
    Pop(Operand<'a>),
    Push(Operand<'a>),
}

