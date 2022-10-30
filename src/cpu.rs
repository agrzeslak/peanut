use crate::register::{
    EflagsRegister, GeneralPurposeRegister, InstructionPointerRegister, SegmentRegister,
};

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
