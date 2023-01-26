use self::{modrm::ModRM, sib::SIB};

mod modrm;
mod sib;

/// May be either 1, 2, or 3 bytes in length. Additional 3-bit opcode field is sometimes encoded
/// within the ModR/M byte.
pub struct PrimaryOpcode(u8, Option<u8>, Option<u8>);

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
    pub prefix_0f: Option<u8>,
    pub primary_opcode: PrimaryOpcode,
    pub secondary_opcode: Option<u8>,
    pub modrm: Option<ModRM>,
    pub sib: Option<SIB>,
    pub displacement: Option<Displacement>,
    pub immediate: Option<Immediate>,
}
