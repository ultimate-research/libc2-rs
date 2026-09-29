#[repr(C)]
pub struct StageKartCircuitFor {
    _0: [u8; 0x3F00],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKartCircuitFor>(), 0x3F00);
        assert_eq!(offset_of!(StageKartCircuitFor, _0), 0x0);
    }
}
