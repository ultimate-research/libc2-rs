use crate::app::StageBaseStorage;

/// The stage where Rathalos is fought.
#[repr(C, align(16))]
pub struct StageBossStageRathalos {
    pub base: StageBaseStorage,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageRathalos>(), 0x740);
        assert_eq!(offset_of!(StageBossStageRathalos, base), 0x0);
    }
}
