use crate::{
    app::{StageBaseStorage, StageDrawModel},
    cpp,
};

use super::StageData;

/// The stage where the credits take place.
#[repr(C, align(16))]
pub struct StageStaffroll {
    pub base: StageBaseStorage,
    pub stage_data: StageData,
    pub stage_data_work: *const StageData,
    pub _768: cpp::UniquePtr<()>,
    pub _770: *mut StageDrawModel,
    pub _778: bool,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageStaffroll>(), 0x780);
        assert_eq!(offset_of!(StageStaffroll, base), 0x0);
        assert_eq!(offset_of!(StageStaffroll, stage_data), 0x738);
        assert_eq!(offset_of!(StageStaffroll, stage_data_work), 0x760);
        assert_eq!(offset_of!(StageStaffroll, _768), 0x768);
        assert_eq!(offset_of!(StageStaffroll, _770), 0x770);
        assert_eq!(offset_of!(StageStaffroll, _778), 0x778);
    }
}
