use crate::app::StageBaseStorage;

/// The stage where players can practice before an online battle.
#[repr(C, align(16))]
pub struct StageShamFight {
    pub base: StageBaseStorage,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageShamFight>(), 0x740);
        assert_eq!(offset_of!(StageShamFight, base), 0x0);
    }
}
