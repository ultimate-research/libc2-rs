#[repr(C)]
pub struct StageBattleAnimalCity {
    _0: [u8; 0x910],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleAnimalCity>(), 0x910);
        assert_eq!(offset_of!(StageBattleAnimalCity, _0), 0x0);
    }
}
