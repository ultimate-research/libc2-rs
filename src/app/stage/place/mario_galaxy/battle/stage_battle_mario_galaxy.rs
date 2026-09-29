#[repr(C)]
pub struct StageBattleMarioGalaxy {
    _0: [u8; 0x810],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleMarioGalaxy>(), 0x810);
        assert_eq!(offset_of!(StageBattleMarioGalaxy, _0), 0x0);
    }
}
