#[repr(C)]
pub struct StageBossStageDracula {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBossStageDracula>(), 0x740);
        assert_eq!(offset_of!(StageBossStageDracula, _0), 0x0);
    }
}
