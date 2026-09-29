#[repr(C)]
pub struct StageFlatZoneX {
    _0: [u8; 0xC80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageFlatZoneX>(), 0xC80);
        assert_eq!(offset_of!(StageFlatZoneX, _0), 0x0);
    }
}
