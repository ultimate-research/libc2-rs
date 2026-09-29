#[repr(C)]
pub struct StagePictochat2 {
    _0: [u8; 0xC80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePictochat2>(), 0xC80);
        assert_eq!(offset_of!(StagePictochat2, _0), 0x0);
    }
}
