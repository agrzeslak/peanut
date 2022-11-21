use bitmaps::Bitmap;

use crate::{instruction::Size, register::Register};

///  Intel manual section 2.1.
///  <http://c-jump.com/CIS77/CPU/x86/X77_0060_mod_reg_r_m_byte.htm>
///
///  7 6  5 4 3 2 1 0 
/// [MOD] [REG] [R/M]
///
/// MOD:
///   - 00: Register indirect addressing mode or SIB with no displacement (when R/M = 100), or
///     displacement only addressing mode (when R/M = 101).
///   - 01: One-byte signed displacement follows addressing mode byte(s).
///   - 10: Four-byte signed displacement follows addressing mode bytes(s).
///   - 11: Register addressing mode.
///
/// REG     8-bit       16-bit      32-bit
/// 000     al          ax          eax
/// 001     cl          cx          ecx
/// 010     dl          dx          edx
/// 011     bl          bx          ebx
/// 100     ah          sp          esp
/// 101     ch          bp          ebp
/// 110     dh          si          esi
/// 111     bh          di          edi
struct ModRM {
    r#mod: Bitmap<2>,
    reg: Bitmap<3>,
    rm: Bitmap<3>,
}

impl ModRM {
    pub fn resolve_register(&self, size: &Size) -> Register {
        use Register::*;
        use Size::*;
        match (self.reg.get(2), self.reg.get(1), self.reg.get(0)) {
            (false, false, false) => {
                match size {
                    Byte => Al,
                    Word => Ax,
                    Dword => Eax,
                    Qword => unimplemented!(), 
                }
            },
            (false, false, true) => {
                match size {
                    Byte => Cl,
                    Word => Cx,
                    Dword => Ecx,
                    Qword => unimplemented!(),
                }
            },
            (false, true, false) => {
                match size {
                    Byte => Dl,
                    Word => Dx,
                    Dword => Edx,
                    Qword => unimplemented!(),
                }
            },
            (false, true, true) => {
                match size {
                    Byte => Bl,
                    Word => Bx,
                    Dword => Ebx,
                    Qword => unimplemented!(),
                }
            },
            (true, false, false) => {
                match size {
                    Byte => Ah,
                    Word => Sp,
                    Dword => Esp,
                    Qword => unimplemented!(),
                }
            },
            (true, false, true) => {
                match size {
                    Byte => Ch,
                    Word => Bp,
                    Dword => Ebp,
                    Qword => unimplemented!(),
                }
            },
            (true, true, false) => {
                match size {
                    Byte => Dh,
                    Word => Si,
                    Dword => Esi,
                    Qword => unimplemented!(),
                }
            },
            (true, true, true) => {
                match size {
                    Byte => Bh,
                    Word => Di,
                    Dword => Edi,
                    Qword => unimplemented!(),
                }
            },
        }
    }
}
