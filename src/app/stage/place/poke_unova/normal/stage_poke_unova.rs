#[repr(C)]
pub struct StagePokeUnova {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeUnova>(), 0x740);
        assert_eq!(offset_of!(StagePokeUnova, _0), 0x0);
    }
}
