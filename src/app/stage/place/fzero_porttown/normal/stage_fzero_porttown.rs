#[repr(C)]
pub struct StageFzeroPorttown {
    _0: [u8; 0x3150],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFzeroPorttown>(), 0x3150);
        assert_eq!(offset_of!(StageFzeroPorttown, _0), 0x0);
    }
}
