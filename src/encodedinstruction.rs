use self::{modrm::ModRM, sib::SIB};

mod modrm;
mod sib;

// TODO: Unclear if this is better than just using a `u8`. Also, if this is used, there must be a
//       way to convert a `u8` into a `Prefix`, without manually writing it out.
pub enum Prefix {
    // Group 1: lock and repeat prefixes.
    Lock,
    Repne,
    Repnz,
    Bnd,
    Rep,
    Repe,
    Repz,

    // Group 2: segment override prefixes and branch hints.
    CsSegmentOverride,
    SsSegmentOverride,
    DsSegmentOverride,
    EsSegmentOverride,
    FsSegmentOverride,
    GsSegmentOverride,
    BranchTaken,
    BranchNotTaken,

    // Group 3: operand-size override prefix.
    OperandSizeOverride,

    // Group 4: address-size override prefix.
    AddressSizeOverride,

    // 9B may be the wait prefix, but unsure.
    Other(u8),
}

impl Prefix {
    pub fn as_u8(&self) -> u8 {
        use Prefix::*;
        match self {
            Lock => 0xF0,
            Repne => 0xF2,
            Repnz => 0xF2,
            Bnd => 0xF2,
            Rep => 0xF3,
            Repe => 0xF3,
            Repz => 0xF3,
            CsSegmentOverride => 0x2E,
            SsSegmentOverride => 0x36,
            DsSegmentOverride => 0x3E,
            EsSegmentOverride => 0x26,
            FsSegmentOverride => 0x64,
            GsSegmentOverride => 0x65,
            BranchTaken => 0x2E,
            BranchNotTaken => 0x3E,
            OperandSizeOverride => 0x66,
            AddressSizeOverride => 0x67,
            Other(n) => *n,
        }
    }
}

/// May be either 1, 2, or 4 bytes.
pub enum Displacement {
    One = 1,
    Two = 2,
    Four = 4,
}

/// May be either 1, 2, or 4 bytes.
pub enum Immediate {
    One = 1,
    Two = 2,
    Four = 4,
}

pub struct Instruction {
    pub prefix: Option<u8>,
    pub prefix_0f: bool,
    pub primary_opcode: u8,
    pub secondary_opcode: Option<u8>,
    pub modrm: Option<ModRM>,
    pub sib: Option<SIB>,
    pub displacement: Option<Displacement>,
    pub immediate: Option<Immediate>,
}
