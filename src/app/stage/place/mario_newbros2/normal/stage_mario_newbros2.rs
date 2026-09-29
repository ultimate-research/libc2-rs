#[repr(C)]
pub struct StageMarioNewBros2 {
    _0: [u8; 0x9A0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioNewBros2>(), 0x9A0);
        assert_eq!(offset_of!(StageMarioNewBros2, _0), 0x0);
    }
}
