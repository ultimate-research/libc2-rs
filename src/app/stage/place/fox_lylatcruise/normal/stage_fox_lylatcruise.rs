#[repr(C)]
pub struct StageFoxLylatCruise {
    _0: [u8; 0x3400],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFoxLylatCruise>(), 0x3400);
        assert_eq!(offset_of!(StageFoxLylatCruise, _0), 0x0);
    }
}
