#[repr(C)]
pub struct StageBossStageFinal1 {
    _0: [u8; 0xA90],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageFinal1>(), 0xA90);
        assert_eq!(offset_of!(StageBossStageFinal1, _0), 0x0);
    }
}
