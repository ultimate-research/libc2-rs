#[repr(C)]
pub struct StageTantanSpring {
    _0: [u8; 0x900],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageTantanSpring>(), 0x900);
        assert_eq!(offset_of!(StageTantanSpring, _0), 0x0);
    }
}
