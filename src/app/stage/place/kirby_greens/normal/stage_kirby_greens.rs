#[repr(C)]
pub struct StageKirbyGreens {
    _0: [u8; 0x11E0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKirbyGreens>(), 0x11E0);
        assert_eq!(offset_of!(StageKirbyGreens, _0), 0x0);
    }
}
