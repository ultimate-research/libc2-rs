use crate::app::StageBaseStorage;

/// The stage where Ganon, The Demon King is fought.
#[repr(C, align(16))]
pub struct StageBossStageGanonBoss {
    pub base: StageBaseStorage,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageGanonBoss>(), 0x740);
        assert_eq!(offset_of!(StageBossStageGanonBoss, base), 0x0);
    }
}
