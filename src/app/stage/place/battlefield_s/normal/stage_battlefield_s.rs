use crate::app::StageBaseStorage;

#[repr(C, align(16))]
pub struct StageBattleFieldS {
    pub base: StageBaseStorage,
    _731: [u8; 0x3F],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFieldS>(), 0x770);
        assert_eq!(offset_of!(StageBattleFieldS, base), 0x0);
        assert_eq!(offset_of!(StageBattleFieldS, _731), 0x731);
    }
}
