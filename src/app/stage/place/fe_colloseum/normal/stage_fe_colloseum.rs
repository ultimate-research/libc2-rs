#[repr(C)]
pub struct StageFEColloseum {
    _0: [u8; 0x7A0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFEColloseum>(), 0x7A0);
        assert_eq!(offset_of!(StageFEColloseum, _0), 0x0);
    }
}
