#[repr(C)]
pub struct StageMetroidNorfair {
    _0: [u8; 0x19F0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMetroidNorfair>(), 0x19F0);
        assert_eq!(offset_of!(StageMetroidNorfair, _0), 0x0);
    }
}
