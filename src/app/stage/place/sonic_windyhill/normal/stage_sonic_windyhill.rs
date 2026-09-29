#[repr(C)]
pub struct StageSonicWindyhill {
    _0: [u8; 0xC90],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSonicWindyhill>(), 0xC90);
        assert_eq!(offset_of!(StageSonicWindyhill, _0), 0x0);
    }
}
