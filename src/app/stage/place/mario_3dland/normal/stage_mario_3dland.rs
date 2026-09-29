#[repr(C)]
pub struct StageMario3DLand {
    _0: [u8; 0xCF0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMario3DLand>(), 0xCF0);
        assert_eq!(offset_of!(StageMario3DLand, _0), 0x0);
    }
}
