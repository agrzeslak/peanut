mod arguments;
mod cpu;
mod error;
mod instruction;
mod machine;
mod memory;
mod newinstruction;
mod register;
mod sib;
mod traits;

use std::fs;

use clap::Parser;
use cpu::Cpu;
use instruction::{Instruction, NasmStr};

use crate::newinstruction::{Displacement, Prefix};

pub fn run() {
    let arguments = arguments::Arguments::parse();
    let file_contents = fs::read_to_string(&arguments.file_path).expect("failed to read file");
    let mut cpu = Cpu::default();
    for line in file_contents.lines() {
        let instruction = Instruction::try_from(&NasmStr(&line)).unwrap();
        (instruction.cpu_function)(&mut cpu, &instruction);
    }
}
