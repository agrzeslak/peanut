mod cpu;
mod error;
mod instruction;
mod memory;
mod register;

use std::fs;

use instruction::{Instruction, NasmStr};

pub fn run() {
    let file_contents = fs::read_to_string("test.asm").expect("failed to read file");
    for line in file_contents.lines() {
         let instruction = Instruction::try_from(&NasmStr(&line)).unwrap();
    }
}
