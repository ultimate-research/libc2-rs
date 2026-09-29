#[repr(C)]
pub struct StagePokeStadium {
    _0: [u8; 0x840],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeStadium>(), 0x840);
        assert_eq!(offset_of!(StagePokeStadium, _0), 0x0);
    }
}
