use memory::Address;
use register::{InstructionPointerRegister, GeneralPurposeRegister, EflagsRegister, SegmentRegister};

mod cpu;
mod instruction;
mod memory;
mod register;


pub struct Offset {
    base: Address,
    offset: i32,
}

pub struct Cpu {
    eax: GeneralPurposeRegister,
    ebx: GeneralPurposeRegister,
    ecx: GeneralPurposeRegister,
    edx: GeneralPurposeRegister,
    esp: GeneralPurposeRegister,
    ebp: GeneralPurposeRegister,
    esi: GeneralPurposeRegister,
    edi: GeneralPurposeRegister,
    eflags: EflagsRegister,
    eip: InstructionPointerRegister,
    cs: SegmentRegister,
    ds: SegmentRegister,
    es: SegmentRegister,
    fs: SegmentRegister,
    gs: SegmentRegister,
    ss: SegmentRegister,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
