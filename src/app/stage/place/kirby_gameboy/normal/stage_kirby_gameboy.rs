#[repr(C)]
pub struct StageKirbyGameboy {
    _0: [u8; 0xB30],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKirbyGameboy>(), 0xB30);
        assert_eq!(offset_of!(StageKirbyGameboy, _0), 0x0);
    }
}
