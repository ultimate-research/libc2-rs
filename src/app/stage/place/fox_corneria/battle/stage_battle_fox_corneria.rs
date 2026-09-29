#[repr(C)]
pub struct StageBattleFoxCorneria {
    _0: [u8; 0x9D0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFoxCorneria>(), 0x9D0);
        assert_eq!(offset_of!(StageBattleFoxCorneria, _0), 0x0);
    }
}
