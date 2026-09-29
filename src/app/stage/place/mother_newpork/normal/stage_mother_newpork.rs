#[repr(C)]
pub struct StageMotherNewpork {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMotherNewpork>(), 0x740);
        assert_eq!(offset_of!(StageMotherNewpork, _0), 0x0);
    }
}
