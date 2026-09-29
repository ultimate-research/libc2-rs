#[repr(C)]
pub struct StageBattleZeldaPirates {
    _0: [u8; 0x790],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleZeldaPirates>(), 0x790);
        assert_eq!(offset_of!(StageBattleZeldaPirates, _0), 0x0);
    }
}
