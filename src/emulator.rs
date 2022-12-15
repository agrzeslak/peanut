use crate::{cpu::Cpu, memory::Ram};

pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
}
