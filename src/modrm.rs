use bitmaps::Bitmap;

use crate::{instruction::Size, register::{Register, Register8, Register16, Register32}};

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
#[derive(Debug, Default)]
struct ModRM {
    pub r#mod: Bitmap<2>,
    pub reg: Bitmap<3>,
    pub rm: Bitmap<3>,
}

impl ModRM {
    pub fn resolve_register(&self, size: &Size) -> Register {
        use Size::*;
        // FIXME: find a better approach than panicking if a qword is provided. Possible separate
        //        size type.
        match (self.reg.get(2), self.reg.get(1), self.reg.get(0)) {
            (false, false, false) => {
                match size {
                    Byte => Register8::Al.into(),
                    Word => Register16::Ax.into(),
                    Dword => Register32::Eax.into(),
                    Qword => unimplemented!(), 
                }
            },
            (false, false, true) => {
                match size {
                    Byte => Register8::Cl.into(),
                    Word => Register16::Cx.into(),
                    Dword => Register32::Ecx.into(),
                    Qword => unimplemented!(),
                }
            },
            (false, true, false) => {
                match size {
                    Byte => Register8::Dl.into(),
                    Word => Register16::Dx.into(),
                    Dword => Register32::Edx.into(),
                    Qword => unimplemented!(),
                }
            },
            (false, true, true) => {
                match size {
                    Byte => Register8::Bl.into(),
                    Word => Register16::Bx.into(),
                    Dword => Register32::Ebx.into(),
                    Qword => unimplemented!(),
                }
            },
            (true, false, false) => {
                match size {
                    Byte => Register8::Ah.into(),
                    Word => Register16::Sp.into(),
                    Dword => Register32::Esp.into(),
                    Qword => unimplemented!(),
                }
            },
            (true, false, true) => {
                match size {
                    Byte => Register8::Ch.into(),
                    Word => Register16::Bp.into(),
                    Dword => Register32::Ebp.into(),
                    Qword => unimplemented!(),
                }
            },
            (true, true, false) => {
                match size {
                    Byte => Register8::Dh.into(),
                    Word => Register16::Si.into(),
                    Dword => Register32::Esi.into(),
                    Qword => unimplemented!(),
                }
            },
            (true, true, true) => {
                match size {
                    Byte => Register8::Bh.into(),
                    Word => Register16::Di.into(),
                    Dword => Register32::Edi.into(),
                    Qword => unimplemented!(),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_register() {
        use Size::*;

        let mut bitmap_000 = Bitmap::<3>::new();
        bitmap_000.set(0, false);
        bitmap_000.set(1, false);
        bitmap_000.set(2, false);

        let mut bitmap_001 = Bitmap::<3>::new();
        bitmap_001.set(0, true);
        bitmap_001.set(1, false);
        bitmap_001.set(2, false);

        let mut bitmap_010 = Bitmap::<3>::new();
        bitmap_010.set(0, false);
        bitmap_010.set(1, true);
        bitmap_010.set(2, false);

        let mut bitmap_011 = Bitmap::<3>::new();
        bitmap_011.set(0, true);
        bitmap_011.set(1, true);
        bitmap_011.set(2, false);

        let mut bitmap_100 = Bitmap::<3>::new();
        bitmap_100.set(0, false);
        bitmap_100.set(1, false);
        bitmap_100.set(2, true);

        let mut bitmap_101 = Bitmap::<3>::new();
        bitmap_101.set(0, true);
        bitmap_101.set(1, false);
        bitmap_101.set(2, true);

        let mut bitmap_110 = Bitmap::<3>::new();
        bitmap_110.set(0, false);
        bitmap_110.set(1, true);
        bitmap_110.set(2, true);

        let mut bitmap_111 = Bitmap::<3>::new();
        bitmap_111.set(0, true);
        bitmap_111.set(1, true);
        bitmap_111.set(2, true);

        let mut modrm = ModRM::default();

        modrm.reg = bitmap_000;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Al.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Ax.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Eax.into());

        modrm.reg = bitmap_001;
        dbg!(&modrm);
        assert_eq!(modrm.resolve_register(&Byte), Register8::Cl.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Cx.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Ecx.into());

        modrm.reg = bitmap_010;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Dl.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Dx.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Edx.into());

        modrm.reg = bitmap_011;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Bl.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Bx.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Ebx.into());

        modrm.reg = bitmap_100;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Ah.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Sp.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Esp.into());

        modrm.reg = bitmap_101;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Ch.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Bp.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Ebp.into());

        modrm.reg = bitmap_110;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Dh.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Si.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Esi.into());

        modrm.reg = bitmap_111;
        assert_eq!(modrm.resolve_register(&Byte), Register8::Bh.into());
        assert_eq!(modrm.resolve_register(&Word), Register16::Di.into());
        assert_eq!(modrm.resolve_register(&Dword), Register32::Edi.into());
    }
}
