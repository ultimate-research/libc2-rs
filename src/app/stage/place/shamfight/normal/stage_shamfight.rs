#[repr(C)]
pub struct StageShamFight {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageShamFight>(), 0x740);
        assert_eq!(offset_of!(StageShamFight, _0), 0x0);
    }
}
