#[repr(C)]
pub struct StageMarioCastleDx {
    _0: [u8; 0x740],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioCastleDx>(), 0x740);
        assert_eq!(offset_of!(StageMarioCastleDx, _0), 0x0);
    }
}
