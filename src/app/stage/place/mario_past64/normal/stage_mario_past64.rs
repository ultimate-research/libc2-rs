#[repr(C)]
pub struct StageMarioPast64 {
    _0: [u8; 0x8E0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioPast64>(), 0x8E0);
        assert_eq!(offset_of!(StageMarioPast64, _0), 0x0);
    }
}
