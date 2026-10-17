use crate::{app::StageBattleBase, cpp};

use super::StageData;

/// The Battlefield stage.
#[repr(C, align(16))]
pub struct StageBattleField {
    pub base: StageBattleBase,
    pub stage_data: StageData,
    pub stage_data_work: *const StageData,
    pub color_correction_controller: cpp::UniquePtr<()>,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleField>(), 0x7B0);
        assert_eq!(offset_of!(StageBattleField, base), 0x0);
        assert_eq!(offset_of!(StageBattleField, stage_data), 0x770);
        assert_eq!(offset_of!(StageBattleField, stage_data_work), 0x798);
        assert_eq!(offset_of!(StageBattleField, color_correction_controller), 0x7A0);
    }
}
