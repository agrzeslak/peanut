use bitmaps::Bitmap;

use crate::memory::Address;

pub struct GeneralPurposeRegister(u32);

pub enum CurrentPrivilegeLevel {
    CPL0,
    CPL1,
    CPL2,
    CPL3,
}

/// Bit #   Mask    Abbreviation    Description     Category    =1              =0
/// 0       0x0001  CF              Carry Flag      Status      CY (Carry)      NC (No Carry)
/// 1       0x0002  Reserved, always 1 in EFLAGS.
/// 2       0x0004  PF              Parity Flag     Status      PE (Parity Even)
pub struct EflagsRegister(Bitmap<64>);

impl EflagsRegister {
    // FIXME: Make this a macro.
    pub fn get_cf(&self) -> bool {
        self.0.get(0)
    }
    pub fn set_cf(&mut self) {
        self.0.set(0, true);
    }
    pub fn clear_cf(&mut self) {
        self.0.set(0, false);
    }
    pub fn get_pf(&self) -> bool {
        self.0.get(2)
    }
    pub fn set_pf(&mut self) {
        self.0.set(2, true);
    }
    pub fn clear_pf(&mut self) {
        self.0.set(2, false);
    }
    pub fn get_af(&self) -> bool {
        self.0.get(4)
    }
    pub fn set_af(&mut self) {
        self.0.set(4, true);
    }
    pub fn clear_af(&mut self) {
        self.0.set(4, false);
    }
    pub fn get_zf(&self) -> bool {
        self.0.get(6)
    }
    pub fn set_zf(&mut self) {
        self.0.set(6, true);
    }
    pub fn clear_zf(&mut self) {
        self.0.set(6, false);
    }
    pub fn get_sf(&self) -> bool {
        self.0.get(7)
    }
    pub fn set_sf(&mut self) {
        self.0.set(7, true);
    }
    pub fn clear_sf(&mut self) {
        self.0.set(7, false);
    }
    pub fn get_tf(&self) -> bool {
        self.0.get(8)
    }
    pub fn set_tf(&mut self) {
        self.0.set(8, true);
    }
    pub fn clear_tf(&mut self) {
        self.0.set(8, false);
    }
    pub fn get_if(&self) -> bool {
        self.0.get(9)
    }
    pub fn set_if(&mut self) {
        self.0.set(9, true);
    }
    pub fn clear_if(&mut self) {
        self.0.set(9, false);
    }
    pub fn get_df(&self) -> bool {
        self.0.get(10)
    }
    pub fn set_df(&mut self) {
        self.0.set(10, true);
    }
    pub fn clear_df(&mut self) {
        self.0.set(10, false);
    }
    pub fn get_of(&self) -> bool {
        self.0.get(11)
    }
    pub fn set_of(&mut self) {
        self.0.set(11, true);
    }
    pub fn clear_of(&mut self) {
        self.0.set(11, false);
    }
    pub fn get_iopl(&self) -> CurrentPrivilegeLevel {
        let first_bit = self.0.get(12);
        let second_bit = self.0.get(13);
        // FIXME: Verify that these bits correspond to the correct privilege levels.
        match (second_bit, first_bit) {
            (false, false) => CurrentPrivilegeLevel::CPL0,
            (false, true) => CurrentPrivilegeLevel::CPL1,
            (true, false) => CurrentPrivilegeLevel::CPL2,
            (true, true) => CurrentPrivilegeLevel::CPL3,
        }
    }
    pub fn set_iopl(&mut self, cpl: CurrentPrivilegeLevel) {
        // FIXME: Verify that these bits correspond to the correct privilege levels.
        let (second_bit, first_bit) = match cpl {
            CurrentPrivilegeLevel::CPL0 => (false, false),
            CurrentPrivilegeLevel::CPL1 => (false, true),
            CurrentPrivilegeLevel::CPL2 => (true, false),
            CurrentPrivilegeLevel::CPL3 => (true, true),
        };
        self.0.set(12, first_bit);
        self.0.set(13, second_bit);
    }
    pub fn get_nt(&self) -> bool {
        self.0.get(14)
    }
    pub fn set_nt(&mut self) {
        self.0.set(14, true);
    }
    pub fn clear_nt(&mut self) {
        self.0.set(14, false);
    }
    pub fn get_rf(&self) -> bool {
        self.0.get(16)
    }
    pub fn set_rf(&mut self) {
        self.0.set(16, true);
    }
    pub fn clear_rf(&mut self) {
        self.0.set(16, false);
    }
    pub fn get_vm(&self) -> bool {
        self.0.get(17)
    }
    pub fn set_vm(&mut self) {
        self.0.set(17, true);
    }
    pub fn clear_vm(&mut self) {
        self.0.set(17, false);
    }
    pub fn get_ac(&self) -> bool {
        self.0.get(18)
    }
    pub fn set_ac(&mut self) {
        self.0.set(18, true);
    }
    pub fn clear_ac(&mut self) {
        self.0.set(18, false);
    }
    pub fn get_vif(&self) -> bool {
        self.0.get(19)
    }
    pub fn set_vif(&mut self) {
        self.0.set(19, true);
    }
    pub fn clear_vif(&mut self) {
        self.0.set(19, false);
    }
    pub fn get_vip(&self) -> bool {
        self.0.get(20)
    }
    pub fn set_vip(&mut self) {
        self.0.set(20, true);
    }
    pub fn clear_vip(&mut self) {
        self.0.set(20, false);
    }
    pub fn get_id(&self) -> bool {
        self.0.get(21)
    }
    pub fn set_id(&mut self) {
        self.0.set(21, true);
    }
    pub fn clear_id(&mut self) {
        self.0.set(21, false);
    }
}

pub struct SegmentRegister(u16);
pub struct InstructionPointerRegister(Address);
pub enum Register {
    Eflags(EflagsRegister),
    Eip(InstructionPointerRegister),
    GeneralPurpose(GeneralPurposeRegister),
    Segment(SegmentRegister),
}
