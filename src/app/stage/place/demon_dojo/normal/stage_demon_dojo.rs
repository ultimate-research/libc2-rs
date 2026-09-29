#[repr(C)]
pub struct StageDemonDojo {
    _0: [u8; 0xBE0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDemonDojo>(), 0xBE0);
        assert_eq!(offset_of!(StageDemonDojo, _0), 0x0);
    }
}
