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
#[derive(Debug, Default)]
struct ModRM {
    pub r#mod: Bitmap<2>,
    pub reg: Bitmap<3>,
    pub rm: Bitmap<3>,
}

impl ModRM {
    pub fn resolve_register(&self, size: &Size) -> Register {
        use Register::*;
        use Size::*;
        // FIXME: find a better approach than panicking if a qword is provided. Possible separate
        //        size type.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_register() {
        use Register::*;
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
        assert_eq!(modrm.resolve_register(&Byte), Al);
        assert_eq!(modrm.resolve_register(&Word), Ax);
        assert_eq!(modrm.resolve_register(&Dword), Eax);

        modrm.reg = bitmap_001;
        dbg!(&modrm);
        assert_eq!(modrm.resolve_register(&Byte), Cl);
        assert_eq!(modrm.resolve_register(&Word), Cx);
        assert_eq!(modrm.resolve_register(&Dword), Ecx);

        modrm.reg = bitmap_010;
        assert_eq!(modrm.resolve_register(&Byte), Dl);
        assert_eq!(modrm.resolve_register(&Word), Dx);
        assert_eq!(modrm.resolve_register(&Dword), Edx);

        modrm.reg = bitmap_011;
        assert_eq!(modrm.resolve_register(&Byte), Bl);
        assert_eq!(modrm.resolve_register(&Word), Bx);
        assert_eq!(modrm.resolve_register(&Dword), Ebx);

        modrm.reg = bitmap_100;
        assert_eq!(modrm.resolve_register(&Byte), Ah);
        assert_eq!(modrm.resolve_register(&Word), Sp);
        assert_eq!(modrm.resolve_register(&Dword), Esp);

        modrm.reg = bitmap_101;
        assert_eq!(modrm.resolve_register(&Byte), Ch);
        assert_eq!(modrm.resolve_register(&Word), Bp);
        assert_eq!(modrm.resolve_register(&Dword), Ebp);

        modrm.reg = bitmap_110;
        assert_eq!(modrm.resolve_register(&Byte), Dh);
        assert_eq!(modrm.resolve_register(&Word), Si);
        assert_eq!(modrm.resolve_register(&Dword), Esi);

        modrm.reg = bitmap_111;
        assert_eq!(modrm.resolve_register(&Byte), Bh);
        assert_eq!(modrm.resolve_register(&Word), Di);
        assert_eq!(modrm.resolve_register(&Dword), Edi);
    }
}
