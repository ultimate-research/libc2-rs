#[repr(C)]
pub struct StageJackMementoes {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageJackMementoes>(), 0x740);
        assert_eq!(offset_of!(StageJackMementoes, _0), 0x0);
    }
}
