#[repr(C)]
pub struct StageSFSuzaku {
    _0: [u8; 0x7E0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSFSuzaku>(), 0x7E0);
        assert_eq!(offset_of!(StageSFSuzaku, _0), 0x0);
    }
}
