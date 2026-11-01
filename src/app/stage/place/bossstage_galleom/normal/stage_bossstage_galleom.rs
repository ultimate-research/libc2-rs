use crate::app::StageBaseStorage;

/// The stage where Galleom is fought.
#[repr(C, align(16))]
pub struct StageBossStageGalleom {
    pub base: StageBaseStorage,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageGalleom>(), 0x740);
        assert_eq!(offset_of!(StageBossStageGalleom, base), 0x0);
    }
}
