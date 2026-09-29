use crate::app::StageBattleBase;

#[repr(C)]
pub struct StageBattleField {
    pub base: StageBattleBase,
    _770: [u8; 0x40],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleField>(), 0x7B0);
        assert_eq!(offset_of!(StageBattleField, base), 0x0);
        assert_eq!(offset_of!(StageBattleField, _770), 0x770);
    }
}
