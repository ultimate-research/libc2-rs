use crate::{app::StageBaseStorage, cpp};

use super::StageData;

/// The stage where Marx is fought.
#[repr(C, align(16))]
pub struct StageBossStageMarx {
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

        assert_eq!(size_of::<StageBossStageMarx>(), 0x770);
        assert_eq!(offset_of!(StageBossStageMarx, base), 0x0);
        assert_eq!(offset_of!(StageBossStageMarx, stage_data), 0x738);
        assert_eq!(offset_of!(StageBossStageMarx, stage_data_work), 0x760);
        assert_eq!(offset_of!(StageBossStageMarx, color_correction_controller), 0x768);
    }
}
