use crate::app::StageBase;

#[repr(C)]
pub struct StageBattleFieldS {
    pub base: StageBase,
    _738: [u8; 0x38],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFieldS>(), 0x770);
        assert_eq!(offset_of!(StageBattleFieldS, base), 0x0);
        assert_eq!(offset_of!(StageBattleFieldS, _738), 0x738);
    }
}
