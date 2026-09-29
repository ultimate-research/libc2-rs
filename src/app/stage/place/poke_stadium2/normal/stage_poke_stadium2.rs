#[repr(C)]
pub struct StagePokeStadium2 {
    _0: [u8; 0x1CC0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeStadium2>(), 0x1CC0);
        assert_eq!(offset_of!(StagePokeStadium2, _0), 0x0);
    }
}
