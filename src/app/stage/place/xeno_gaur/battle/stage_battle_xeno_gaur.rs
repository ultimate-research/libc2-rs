#[repr(C)]
pub struct StageBattleXenoGaur {
    _0: [u8; 0x8B0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleXenoGaur>(), 0x8B0);
        assert_eq!(offset_of!(StageBattleXenoGaur, _0), 0x0);
    }
}
