#[repr(C)]
pub struct StageZeldaHyrule {
    _0: [u8; 0x7D0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaHyrule>(), 0x7D0);
        assert_eq!(offset_of!(StageZeldaHyrule, _0), 0x0);
    }
}
