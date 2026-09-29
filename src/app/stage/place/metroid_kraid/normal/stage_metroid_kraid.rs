#[repr(C)]
pub struct StageMetroidKraid {
    _0: [u8; 0x7D0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMetroidKraid>(), 0x7D0);
        assert_eq!(offset_of!(StageMetroidKraid, _0), 0x0);
    }
}
