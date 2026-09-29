#[repr(C)]
pub struct StageMarioCastle64 {
    _0: [u8; 0x810],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioCastle64>(), 0x810);
        assert_eq!(offset_of!(StageMarioCastle64, _0), 0x0);
    }
}
