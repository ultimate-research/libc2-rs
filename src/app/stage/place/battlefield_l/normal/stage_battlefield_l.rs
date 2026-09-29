use crate::app::StageBase;

#[repr(C)]
pub struct StageBattleFieldL {
    pub base: StageBase,
    _738: [u8; 0x38],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFieldL>(), 0x770);
        assert_eq!(offset_of!(StageBattleFieldL, base), 0x0);
        assert_eq!(offset_of!(StageBattleFieldL, _738), 0x738);
    }
}
