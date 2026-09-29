#[repr(C)]
pub struct StageKirbyHalberd {
    _0: [u8; 0xAC0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKirbyHalberd>(), 0xAC0);
        assert_eq!(offset_of!(StageKirbyHalberd, _0), 0x0);
    }
}
