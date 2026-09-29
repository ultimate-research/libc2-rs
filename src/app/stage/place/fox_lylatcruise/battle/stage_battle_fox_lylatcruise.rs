#[repr(C)]
pub struct StageBattleFoxLylatCruise {
    _0: [u8; 0xAE0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFoxLylatCruise>(), 0xAE0);
        assert_eq!(offset_of!(StageBattleFoxLylatCruise, _0), 0x0);
    }
}
