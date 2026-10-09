use crate::app::StageBaseStorage;

#[repr(C, align(16))]
pub struct StageBattleFieldL {
    pub base: StageBaseStorage,
    _731: [u8; 0x3F],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFieldL>(), 0x770);
        assert_eq!(offset_of!(StageBattleFieldL, base), 0x0);
        assert_eq!(offset_of!(StageBattleFieldL, _731), 0x731);
    }
}
