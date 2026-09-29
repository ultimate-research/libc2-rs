#[repr(C)]
pub struct StageKirbyCave {
    _0: [u8; 0xAD0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKirbyCave>(), 0xAD0);
        assert_eq!(offset_of!(StageKirbyCave, _0), 0x0);
    }
}
