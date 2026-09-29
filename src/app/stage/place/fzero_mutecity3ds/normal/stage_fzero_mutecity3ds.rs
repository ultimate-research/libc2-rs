#[repr(C)]
pub struct StageFzeroMutecity3DS {
    _0: [u8; 0xAB0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFzeroMutecity3DS>(), 0xAB0);
        assert_eq!(offset_of!(StageFzeroMutecity3DS, _0), 0x0);
    }
}
