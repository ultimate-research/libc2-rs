#[repr(C)]
pub struct StageIcarusSkyWorld {
    _0: [u8; 0x8A0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageIcarusSkyWorld>(), 0x8A0);
        assert_eq!(offset_of!(StageIcarusSkyWorld, _0), 0x0);
    }
}
