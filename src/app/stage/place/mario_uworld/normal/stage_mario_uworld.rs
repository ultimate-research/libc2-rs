#[repr(C)]
pub struct StageMarioUworld {
    _0: [u8; 0xAC0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioUworld>(), 0xAC0);
        assert_eq!(offset_of!(StageMarioUworld, _0), 0x0);
    }
}
