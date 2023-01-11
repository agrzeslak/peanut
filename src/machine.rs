use crate::{cpu::Cpu, memory::Ram};

#[derive(Clone, Debug, Default)]
pub struct Machine {
    cpu: Cpu,
    ram: Ram,
}
