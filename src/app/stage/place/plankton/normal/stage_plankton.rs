#[repr(C)]
pub struct StagePlankton {
    _0: [u8; 0x17980],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePlankton>(), 0x17980);
        assert_eq!(offset_of!(StagePlankton, _0), 0x0);
    }
}
