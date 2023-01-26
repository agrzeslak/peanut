use crate::{cpu::Cpu, memory::Memory};

#[derive(Clone, Debug, Default)]
pub struct Machine {
    cpu: Cpu,
    ram: Memory,
}
