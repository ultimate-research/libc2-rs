#[repr(C)]
pub struct StageMotherFourside {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMotherFourside>(), 0x740);
        assert_eq!(offset_of!(StageMotherFourside, _0), 0x0);
    }
}
