#[repr(C)]
pub struct StageBattlePokeStadium2 {
    _0: [u8; 0x8D0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattlePokeStadium2>(), 0x8D0);
        assert_eq!(offset_of!(StageBattlePokeStadium2, _0), 0x0);
    }
}
