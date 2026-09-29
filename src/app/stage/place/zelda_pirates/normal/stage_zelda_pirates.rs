#[repr(C)]
pub struct StageZeldaPirates {
    _0: [u8; 0x8B0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageZeldaPirates>(), 0x8B0);
        assert_eq!(offset_of!(StageZeldaPirates, _0), 0x0);
    }
}
