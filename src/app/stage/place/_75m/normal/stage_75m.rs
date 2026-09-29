#[repr(C)]
pub struct Stage75m {
    _0: [u8; 0xA00],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<Stage75m>(), 0xA00);
        assert_eq!(offset_of!(Stage75m, _0), 0x0);
    }
}
