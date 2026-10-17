use crate::{app::StageBaseStorage, cpp};

use super::StageData;

/// The Big Battlefield stage.
#[repr(C, align(16))]
pub struct StageBattleFieldL {
    pub base: StageBaseStorage,
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

        assert_eq!(size_of::<StageBattleFieldL>(), 0x770);
        assert_eq!(offset_of!(StageBattleFieldL, base), 0x0);
        assert_eq!(offset_of!(StageBattleFieldL, stage_data), 0x738);
        assert_eq!(offset_of!(StageBattleFieldL, stage_data_work), 0x760);
        assert_eq!(offset_of!(StageBattleFieldL, color_correction_controller), 0x768);
    }
}
