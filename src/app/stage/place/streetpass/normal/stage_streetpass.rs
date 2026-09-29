#[repr(C)]
pub struct StageStreetPass {
    _0: [u8; 0x960],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageStreetPass>(), 0x960);
        assert_eq!(offset_of!(StageStreetPass, _0), 0x0);
    }
}
