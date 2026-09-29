#[repr(C)]
pub struct StageDollyStadium {
    _0: [u8; 0x810],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDollyStadium>(), 0x810);
        assert_eq!(offset_of!(StageDollyStadium, _0), 0x0);
    }
}
