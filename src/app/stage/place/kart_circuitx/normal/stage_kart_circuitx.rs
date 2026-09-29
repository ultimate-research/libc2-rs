#[repr(C)]
pub struct StageKartCircuitX {
    _0: [u8; 0x8F0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKartCircuitX>(), 0x8F0);
        assert_eq!(offset_of!(StageKartCircuitX, _0), 0x0);
    }
}
