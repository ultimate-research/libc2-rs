#[repr(C)]
pub struct StageBossStageFinal2 {
    _0: [u8; 0x950],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageFinal2>(), 0x950);
        assert_eq!(offset_of!(StageBossStageFinal2, _0), 0x0);
    }
}
