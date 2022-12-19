use crate::{cpu::Cpu, memory::Ram};

pub struct Machine {
    cpu: Cpu,
    ram: Ram,
}
