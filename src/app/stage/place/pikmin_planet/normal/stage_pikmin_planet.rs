#[repr(C)]
pub struct StagePikminPlanet {
    _0: [u8; 0xA10],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePikminPlanet>(), 0xA10);
        assert_eq!(offset_of!(StagePikminPlanet, _0), 0x0);
    }
}
