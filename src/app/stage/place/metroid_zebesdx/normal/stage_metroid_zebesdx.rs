#[repr(C)]
pub struct StageMetroidZebesDx {
    _0: [u8; 0xFE0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMetroidZebesDx>(), 0xFE0);
        assert_eq!(offset_of!(StageMetroidZebesDx, _0), 0x0);
    }
}
