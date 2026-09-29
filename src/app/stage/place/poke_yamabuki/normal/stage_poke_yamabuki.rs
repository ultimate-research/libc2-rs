#[repr(C)]
pub struct StagePokeYamabuki {
    _0: [u8; 0xCB0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeYamabuki>(), 0xCB0);
        assert_eq!(offset_of!(StagePokeYamabuki, _0), 0x0);
    }
}
