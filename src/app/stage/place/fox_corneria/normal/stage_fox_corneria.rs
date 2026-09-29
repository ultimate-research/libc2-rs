#[repr(C)]
pub struct StageFoxCorneria {
    _0: [u8; 0xC60],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFoxCorneria>(), 0xC60);
        assert_eq!(offset_of!(StageFoxCorneria, _0), 0x0);
    }
}
