#[repr(C)]
pub struct StageBossStageFinal3 {
    _0: [u8; 0x770],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageFinal3>(), 0x770);
        assert_eq!(offset_of!(StageBossStageFinal3, _0), 0x0);
    }
}
